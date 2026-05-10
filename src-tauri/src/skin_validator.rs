use crate::skin::SkinConfig;
use image::GenericImageView;
use std::collections::HashSet;
use std::path::Path;

pub const REQUIRED_STATES: [&str; 11] = [
    "idle", "walk", "click", "drag", "sleep", "wake", "hover", "happy", "sad", "surprised", "wave",
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationError {
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

pub fn validate_skin(skin_dir: &Path, skin: &SkinConfig) -> ValidationResult {
    let mut errors: Vec<ValidationError> = Vec::new();

    // 1. coreProfile 检查
    if skin.core_profile != "rich-v1" {
        errors.push(ValidationError {
            kind: "profile".into(),
            message: format!(
                "Expected coreProfile 'rich-v1', got '{}'",
                skin.core_profile
            ),
        });
    }

    // 1.5. canvas 字段完整性检查（防止 panic）
    if skin.canvas.width == 0 || skin.canvas.height == 0 {
        errors.push(ValidationError {
            kind: "invalid_canvas".into(),
            message: format!(
                "Canvas dimensions invalid: width={}, height={} (must be > 0)",
                skin.canvas.width, skin.canvas.height
            ),
        });
    }

    // 2. 强制键检查
    let skin_states: HashSet<&str> = skin.states.keys().map(|s| s.as_str()).collect();
    for required in REQUIRED_STATES.iter() {
        if !skin_states.contains(required) {
            errors.push(ValidationError {
                kind: "missing_state".into(),
                message: format!("Missing required state: {}", required),
            });
        }
    }

    // 3. 帧文件存在性检查
    for (state_name, state) in &skin.states {
        for frame in &state.frames {
            let frame_path = skin_dir.join(frame);
            if !frame_path.exists() {
                errors.push(ValidationError {
                    kind: "missing_frame".into(),
                    message: format!("State '{}' frame '{}' not found", state_name, frame),
                });
            }
        }
    }

    // 4. PNG 格式有效性检查（文件头）
    for (state_name, state) in &skin.states {
        for frame in &state.frames {
            let frame_path = skin_dir.join(frame);
            if frame_path.exists() {
                if let Ok(bytes) = std::fs::read(&frame_path) {
                    // PNG 签名: 89 50 4E 47 0D 0A 1A 0A
                    const PNG_SIGNATURE: [u8; 8] =
                        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
                    if bytes.len() < 8 || bytes[0..8] != PNG_SIGNATURE {
                        errors.push(ValidationError {
                            kind: "invalid_png".into(),
                            message: format!(
                                "State '{}' frame '{}' is not a valid PNG",
                                state_name, frame
                            ),
                        });
                    }
                }
            }
        }
    }

    // 5. 帧尺寸与 canvas 一致校验
    for (state_name, state) in &skin.states {
        for frame in &state.frames {
            let frame_path = skin_dir.join(frame);
            if frame_path.exists() {
                if let Ok(img) = image::open(&frame_path) {
                    let (width, height) = img.dimensions();
                    if width != skin.canvas.width || height != skin.canvas.height {
                        errors.push(ValidationError {
                            kind: "size_mismatch".into(),
                            message: format!(
                                "State '{}' frame '{}' size ({}, {}) != canvas ({}, {})",
                                state_name, frame, width, height, skin.canvas.width, skin.canvas.height
                            ),
                        });
                    }
                } else {
                    // image::open 失败（非图片格式）
                    errors.push(ValidationError {
                        kind: "invalid_image".into(),
                        message: format!(
                            "State '{}' frame '{}' cannot be opened as image",
                            state_name, frame
                        ),
                    });
                }
            }
        }
    }

    ValidationResult {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skin::{SkinConfig, Canvas, Anchor, State};
    use std::collections::HashMap;

    fn make_test_skin() -> SkinConfig {
        SkinConfig {
            schema_version: "1.0".into(),
            core_profile: "rich-v1".into(),
            id: "test".into(),
            name: "Test".into(),
            author: "test".into(),
            canvas: Canvas { width: 512, height: 512 },
            anchor: Anchor { x: 256, y: 512 },
            states: HashMap::new(),
        }
    }

    #[test]
    fn test_required_states_count() {
        assert_eq!(REQUIRED_STATES.len(), 11);
        for state in REQUIRED_STATES.iter() {
            assert!(!state.is_empty());
        }
    }

    #[test]
    fn test_validate_all_states_present() {
        let mut skin = make_test_skin();
        for state in REQUIRED_STATES.iter() {
            skin.states.insert(state.to_string(), State {
                frames: vec!["frames/test.png".into()],
                fps: Some(1),
                duration_ms: None,
                loop_type: Some("loop".into()),
            });
        }
        let result = validate_skin(Path::new("/tmp"), &skin);
        // No missing_frame error since /tmp doesn't have frames, but no missing_state
        assert!(!result.errors.iter().any(|e| e.kind == "missing_state"));
        assert!(!result.errors.iter().any(|e| e.kind == "profile"));
    }

    #[test]
    fn test_validate_missing_states() {
        let skin = make_test_skin();
        let result = validate_skin(Path::new("/tmp"), &skin);
        assert!(!result.valid);
        assert_eq!(result.errors.iter().filter(|e| e.kind == "missing_state").count(), 11);
    }

    #[test]
    fn test_validate_profile_mismatch() {
        let skin = SkinConfig {
            core_profile: "wrong-profile".into(),
            ..make_test_skin()
        };
        let result = validate_skin(Path::new("/tmp"), &skin);
        assert!(result.errors.iter().any(|e| e.kind == "profile"));
    }

    #[test]
    fn test_validate_canvas_zero() {
        let skin = SkinConfig {
            canvas: Canvas { width: 0, height: 512 },
            ..make_test_skin()
        };
        let result = validate_skin(Path::new("/tmp"), &skin);
        assert!(result.errors.iter().any(|e| e.kind == "invalid_canvas"));
    }
}