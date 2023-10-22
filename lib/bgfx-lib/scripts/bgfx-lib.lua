--
-- Copyright 2010-2023 Branimir Karadzic. All rights reserved.
-- License: https://github.com/bkaradzic/bx#license-bsd-2-clause
--

project "bgfx-lib"
	kind "StaticLib"

	includedirs {
		path.join(BX_DIR, "include"),
        path.join(BIMG_DIR, "include"),
		path.join(BGFX_DIR, "include"),
		path.join(BGFX_DIR, "3rdparty"),
	}

	files {
		path.join(BGFX_LIB_DIR, "src/**.h"),
		path.join(BGFX_LIB_DIR, "src/**.cpp"),
        path.join(BGFX_LIB_DIR, "src/**/*.h"),
        path.join(BGFX_LIB_DIR, "src/**/*.cpp"),
	}

	configuration {}

	removeflags {
		"FloatFast", -- astc-encoder doesn't work with it.
	}

	configuration { "linux-*" }
		buildoptions {
			"-fPIC",
		}

	configuration { "Debug" }
		defines {
			"BX_CONFIG_DEBUG=1",
		}

	configuration { "Release" }
		defines {
			"BX_CONFIG_DEBUG=0",
		}

	configuration {}
