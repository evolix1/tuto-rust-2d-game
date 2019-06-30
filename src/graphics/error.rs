use error_chain::error_chain;
pub use error_chain::bail; // Re-export

use sdl2::render::{TextureValueError, TargetRenderError};

use super::sprite::SpriteId;


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        //
        MissingTexture(index: usize) {
            description("missing texture"),
            display("missing texture (index={})", index),
        }

        MissingSprite(id: SpriteId) {
            description("missing sprite"),
            display("missing sprite (id={:?})", id),
        }

        // `sdl2` crate has sooooo many ways to handle errors, it is comic as
        // this point
        SdlUnknownError(msg: String) {
            description("SDL error"),
            display("SDL error: {}", msg),
        }

        SdlTextureError(error: TextureValueError) {
            description("SDL texture error"),
            display("SDL texture error: {}", error),
        }

        SdlRenderError(error: TargetRenderError) {
            description("SDL render error"),
            display("SDL render error: {}", error),
        }
    }
}


pub trait IntoSdlError {
    type Output;

    fn into_sdl_error(self) -> Result<Self::Output>;
}


impl<T> IntoSdlError for std::result::Result<T, String> {
    type Output = T;

    fn into_sdl_error(self) -> Result<T> {
        self.map_err(|msg| ErrorKind::SdlUnknownError(msg).into())
    }
}


impl<T> IntoSdlError for std::result::Result<T, TextureValueError> {
    type Output = T;

    fn into_sdl_error(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(TextureValueError::SdlError(msg))
                => Err(ErrorKind::SdlUnknownError(msg).into()),
            Err(error) => bail!(ErrorKind::SdlTextureError(error)),
        }
    }
}


impl<T> IntoSdlError for std::result::Result<T, TargetRenderError> {
    type Output = T;

    fn into_sdl_error(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            // NOTE: reading the source, it is only a wrapper around a string
            Err(TargetRenderError::SdlError(wrapper)) =>
                Err(ErrorKind::SdlUnknownError(format!("{}", wrapper)).into()),
            Err(error) => bail!(ErrorKind::SdlRenderError(error)),
        }
    }
}
