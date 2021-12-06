use winapi::ctypes::*;
use winapi::shared::d3d9::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;
use winapi::shared::guiddef::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::wingdi::{PALETTEENTRY, RGNDATA};
use winapi::um::winnt::{HANDLE, HRESULT, VOID};

use super::d9_proxy::{leak, OnDestroy, OnRender, OnReset};

static mut D3D9_DEVICE: *mut IDirect3DDevice9 = 0 as *mut _;
static mut RENDER_HOOK_FN: Option<OnRender> = None;
static mut RESET_HOOK_FN: Option<OnReset> = None;
static mut DESTROY_HOOK_FN: Option<OnDestroy> = None;
static mut THIS_DEVICE: *mut IDirect3DDevice9 = 0 as *mut _;

#[repr(C)]
struct Device {
    vftable: *mut IDirect3DDevice9Vtbl,
}

pub fn set_proxy(
    origin: *mut IDirect3DDevice9,
    render: Option<OnRender>,
    reset: Option<OnReset>,
    destroy: Option<OnDestroy>,
) -> *mut IDirect3DDevice9 {
    unsafe {
        let vftable = create_vftable();

        let device = Device {
            vftable: leak(vftable),
        };

        let ptr = leak(device) as *mut _;

        D3D9_DEVICE = origin;
        RENDER_HOOK_FN = render;
        RESET_HOOK_FN = reset;
        DESTROY_HOOK_FN = destroy;
        THIS_DEVICE = ptr;

        ptr
    }
}

fn delete(obj: *mut Device) {
    unsafe {
        if let Some(func) = DESTROY_HOOK_FN {
            func(device());
        }

        let device = Box::from_raw(obj);
        let vftable = Box::from_raw(device.vftable);
        drop(vftable);
        drop(device);
    }
}

pub fn device() -> &'static mut IDirect3DDevice9 {
    unsafe { &mut *THIS_DEVICE }
}

fn create_vftable() -> IDirect3DDevice9Vtbl {
    let parent = IUnknownVtbl {
        QueryInterface,
        AddRef,
        Release,
    };

    IDirect3DDevice9Vtbl {
        parent,
        TestCooperativeLevel,
        GetAvailableTextureMem,
        EvictManagedResources,
        GetDirect3D,
        GetDeviceCaps,
        GetDisplayMode,
        GetCreationParameters,
        SetCursorProperties,
        SetCursorPosition,
        ShowCursor,
        CreateAdditionalSwapChain,
        GetSwapChain,
        GetNumberOfSwapChains,
        Reset,
        Present,
        GetBackBuffer,
        GetRasterStatus,
        SetDialogBoxMode,
        SetGammaRamp,
        GetGammaRamp,
        CreateTexture,
        CreateVolumeTexture,
        CreateCubeTexture,
        CreateVertexBuffer,
        CreateIndexBuffer,
        CreateRenderTarget,
        CreateDepthStencilSurface,
        UpdateSurface,
        UpdateTexture,
        GetRenderTargetData,
        GetFrontBufferData,
        StretchRect,
        ColorFill,
        CreateOffscreenPlainSurface,
        SetRenderTarget,
        GetRenderTarget,
        SetDepthStencilSurface,
        GetDepthStencilSurface,
        BeginScene,
        EndScene,
        Clear,
        SetTransform,
        GetTransform,
        MultiplyTransform,
        SetViewport,
        GetViewport,
        SetMaterial,
        GetMaterial,
        SetLight,
        GetLight,
        LightEnable,
        GetLightEnable,
        SetClipPlane,
        GetClipPlane,
        SetRenderState,
        GetRenderState,
        CreateStateBlock,
        BeginStateBlock,
        EndStateBlock,
        SetClipStatus,
        GetClipStatus,
        GetTexture,
        SetTexture,
        GetTextureStageState,
        SetTextureStageState,
        GetSamplerState,
        SetSamplerState,
        ValidateDevice,
        SetPaletteEntries,
        GetPaletteEntries,
        SetCurrentTexturePalette,
        GetCurrentTexturePalette,
        SetScissorRect,
        GetScissorRect,
        SetSoftwareVertexProcessing,
        GetSoftwareVertexProcessing,
        SetNPatchMode,
        GetNPatchMode,
        DrawPrimitive,
        DrawIndexedPrimitive,
        DrawPrimitiveUP,
        DrawIndexedPrimitiveUP,
        ProcessVertices,
        CreateVertexDeclaration,
        SetVertexDeclaration,
        GetVertexDeclaration,
        SetFVF,
        GetFVF,
        CreateVertexShader,
        SetVertexShader,
        GetVertexShader,
        SetVertexShaderConstantF,
        GetVertexShaderConstantF,
        SetVertexShaderConstantI,
        GetVertexShaderConstantI,
        SetVertexShaderConstantB,
        GetVertexShaderConstantB,
        SetStreamSource,
        GetStreamSource,
        SetStreamSourceFreq,
        GetStreamSourceFreq,
        SetIndices,
        GetIndices,
        CreatePixelShader,
        SetPixelShader,
        GetPixelShader,
        SetPixelShaderConstantF,
        GetPixelShaderConstantF,
        SetPixelShaderConstantI,
        GetPixelShaderConstantI,
        SetPixelShaderConstantB,
        GetPixelShaderConstantB,
        DrawRectPatch,
        DrawTriPatch,
        DeletePatch,
        CreateQuery,
    }
}

unsafe extern "system" fn QueryInterface(
    this: *mut IUnknown,
    riid: REFIID,
    ppvObject: *mut *mut c_void,
) -> HRESULT {
    *ppvObject = std::ptr::null_mut();
    let result = (*D3D9_DEVICE).QueryInterface(riid, ppvObject);

    if result == 0 {
        *ppvObject = this as *mut c_void;
    }

    result
}

unsafe extern "system" fn AddRef(_this: *mut IUnknown) -> ULONG {
    (*D3D9_DEVICE).AddRef()
}

unsafe extern "system" fn Release(this: *mut IUnknown) -> ULONG {
    (*D3D9_DEVICE).AddRef();
    let result = (*D3D9_DEVICE).Release();

    if result == 1 {
        delete(this as *mut Device);
    }

    return (*D3D9_DEVICE).Release();
}

unsafe extern "system" fn TestCooperativeLevel(_this: *mut IDirect3DDevice9) -> HRESULT {
    (*D3D9_DEVICE).TestCooperativeLevel()
}

unsafe extern "system" fn GetAvailableTextureMem(_this: *mut IDirect3DDevice9) -> UINT {
    (*D3D9_DEVICE).GetAvailableTextureMem()
}

unsafe extern "system" fn EvictManagedResources(_this: *mut IDirect3DDevice9) -> HRESULT {
    (*D3D9_DEVICE).EvictManagedResources()
}

unsafe extern "system" fn GetDirect3D(
    _this: *mut IDirect3DDevice9,
    ppD3D9: *mut *mut IDirect3D9,
) -> HRESULT {
    (*D3D9_DEVICE).GetDirect3D(ppD3D9)
}

unsafe extern "system" fn GetDeviceCaps(
    _this: *mut IDirect3DDevice9,
    pCaps: *mut D3DCAPS9,
) -> HRESULT {
    (*D3D9_DEVICE).GetDeviceCaps(pCaps)
}

unsafe extern "system" fn GetDisplayMode(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    pMode: *mut D3DDISPLAYMODE,
) -> HRESULT {
    (*D3D9_DEVICE).GetDisplayMode(iSwapChain, pMode)
}

unsafe extern "system" fn GetCreationParameters(
    _this: *mut IDirect3DDevice9,
    pParameters: *mut D3DDEVICE_CREATION_PARAMETERS,
) -> HRESULT {
    (*D3D9_DEVICE).GetCreationParameters(pParameters)
}

unsafe extern "system" fn SetCursorProperties(
    _this: *mut IDirect3DDevice9,
    XHotSpot: UINT,
    YHotSpot: UINT,
    pCursorBitmap: *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).SetCursorProperties(XHotSpot, YHotSpot, pCursorBitmap)
}

unsafe extern "system" fn SetCursorPosition(
    _this: *mut IDirect3DDevice9,
    X: INT,
    Y: INT,
    Flags: DWORD,
) {
    (*D3D9_DEVICE).SetCursorPosition(X, Y, Flags)
}

unsafe extern "system" fn ShowCursor(_this: *mut IDirect3DDevice9, bShow: BOOL) -> BOOL {
    (*D3D9_DEVICE).ShowCursor(bShow)
}

unsafe extern "system" fn CreateAdditionalSwapChain(
    _this: *mut IDirect3DDevice9,
    pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
    pSwapChain: *mut *mut IDirect3DSwapChain9,
) -> HRESULT {
    (*D3D9_DEVICE).CreateAdditionalSwapChain(pPresentationParameters, pSwapChain)
}

unsafe extern "system" fn GetSwapChain(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    pSwapChain: *mut *mut IDirect3DSwapChain9,
) -> HRESULT {
    (*D3D9_DEVICE).GetSwapChain(iSwapChain, pSwapChain)
}

unsafe extern "system" fn GetNumberOfSwapChains(_this: *mut IDirect3DDevice9) -> UINT {
    (*D3D9_DEVICE).GetNumberOfSwapChains()
}

unsafe extern "system" fn Reset(
    _this: *mut IDirect3DDevice9,
    pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
) -> HRESULT {
    if let Some(func) = RESET_HOOK_FN {
        func(device(), 0);
    }

    let result = (*D3D9_DEVICE).Reset(pPresentationParameters);

    if result == 0 {
        if let Some(func) = RESET_HOOK_FN {
            func(device(), 1);
        }
    } else {
        println!(
            "Device::Reset() result {:X}.\nD3D9_DEVICE: {:?}\nthis: {:?}\nTHIS_DEVICE: {:?}",
            result, D3D9_DEVICE, _this, THIS_DEVICE
        );
    }

    return result;
}

unsafe extern "system" fn Present(
    _this: *mut IDirect3DDevice9,
    pSourceRect: *const RECT,
    pDestRect: *const RECT,
    hDestWindowOverride: HWND,
    pDirtyRegion: *const RGNDATA,
) -> HRESULT {
    (*D3D9_DEVICE).Present(pSourceRect, pDestRect, hDestWindowOverride, pDirtyRegion)
}

unsafe extern "system" fn GetBackBuffer(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    iBackBuffer: UINT,
    Type: D3DBACKBUFFER_TYPE,
    ppBackBuffer: *mut *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).GetBackBuffer(iSwapChain, iBackBuffer, Type, ppBackBuffer)
}

unsafe extern "system" fn GetRasterStatus(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    pRasterStatus: *mut D3DRASTER_STATUS,
) -> HRESULT {
    (*D3D9_DEVICE).GetRasterStatus(iSwapChain, pRasterStatus)
}

unsafe extern "system" fn SetDialogBoxMode(
    _this: *mut IDirect3DDevice9,
    bEnableDialogs: BOOL,
) -> HRESULT {
    (*D3D9_DEVICE).SetDialogBoxMode(bEnableDialogs)
}

unsafe extern "system" fn SetGammaRamp(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    Flags: DWORD,
    pRamp: *const D3DGAMMARAMP,
) {
    (*D3D9_DEVICE).SetGammaRamp(iSwapChain, Flags, pRamp)
}

unsafe extern "system" fn GetGammaRamp(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    pRamp: *mut D3DGAMMARAMP,
) {
    (*D3D9_DEVICE).GetGammaRamp(iSwapChain, pRamp)
}

unsafe extern "system" fn CreateTexture(
    _this: *mut IDirect3DDevice9,
    Width: UINT,
    Height: UINT,
    Levels: UINT,
    Usage: DWORD,
    Format: D3DFORMAT,
    Pool: D3DPOOL,
    ppTexture: *mut *mut IDirect3DTexture9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateTexture(
        Width,
        Height,
        Levels,
        Usage,
        Format,
        Pool,
        ppTexture,
        pSharedHandle,
    )
}

unsafe extern "system" fn CreateVolumeTexture(
    _this: *mut IDirect3DDevice9,
    Width: UINT,
    Height: UINT,
    Depth: UINT,
    Levels: UINT,
    Usage: DWORD,
    Format: D3DFORMAT,
    Pool: D3DPOOL,
    ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateVolumeTexture(
        Width,
        Height,
        Depth,
        Levels,
        Usage,
        Format,
        Pool,
        ppVolumeTexture,
        pSharedHandle,
    )
}

unsafe extern "system" fn CreateCubeTexture(
    _this: *mut IDirect3DDevice9,
    EdgeLength: UINT,
    Levels: UINT,
    Usage: DWORD,
    Format: D3DFORMAT,
    Pool: D3DPOOL,
    ppCubeTexture: *mut *mut IDirect3DCubeTexture9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateCubeTexture(
        EdgeLength,
        Levels,
        Usage,
        Format,
        Pool,
        ppCubeTexture,
        pSharedHandle,
    )
}

unsafe extern "system" fn CreateVertexBuffer(
    _this: *mut IDirect3DDevice9,
    Length: UINT,
    Usage: DWORD,
    FVF: DWORD,
    Pool: D3DPOOL,
    ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateVertexBuffer(Length, Usage, FVF, Pool, ppVertexBuffer, pSharedHandle)
}

unsafe extern "system" fn CreateIndexBuffer(
    _this: *mut IDirect3DDevice9,
    Length: UINT,
    Usage: DWORD,
    Format: D3DFORMAT,
    Pool: D3DPOOL,
    ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateIndexBuffer(Length, Usage, Format, Pool, ppIndexBuffer, pSharedHandle)
}

unsafe extern "system" fn CreateRenderTarget(
    _this: *mut IDirect3DDevice9,
    Width: UINT,
    Height: UINT,
    Format: D3DFORMAT,
    MultiSample: D3DMULTISAMPLE_TYPE,
    MultisampleQuality: DWORD,
    Lockable: BOOL,
    ppSurface: *mut *mut IDirect3DSurface9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateRenderTarget(
        Width,
        Height,
        Format,
        MultiSample,
        MultisampleQuality,
        Lockable,
        ppSurface,
        pSharedHandle,
    )
}

unsafe extern "system" fn CreateDepthStencilSurface(
    _this: *mut IDirect3DDevice9,
    Width: UINT,
    Height: UINT,
    Format: D3DFORMAT,
    MultiSample: D3DMULTISAMPLE_TYPE,
    MultisampleQuality: DWORD,
    Discard: BOOL,
    ppSurface: *mut *mut IDirect3DSurface9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateDepthStencilSurface(
        Width,
        Height,
        Format,
        MultiSample,
        MultisampleQuality,
        Discard,
        ppSurface,
        pSharedHandle,
    )
}

unsafe extern "system" fn UpdateSurface(
    _this: *mut IDirect3DDevice9,
    pSourceSurface: *mut IDirect3DSurface9,
    pSourceRect: *const RECT,
    pDestinationSurface: *mut IDirect3DSurface9,
    pDestPoint: *const POINT,
) -> HRESULT {
    (*D3D9_DEVICE).UpdateSurface(pSourceSurface, pSourceRect, pDestinationSurface, pDestPoint)
}

unsafe extern "system" fn UpdateTexture(
    _this: *mut IDirect3DDevice9,
    pSourceTexture: *mut IDirect3DBaseTexture9,
    pDestinationTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT {
    (*D3D9_DEVICE).UpdateTexture(pSourceTexture, pDestinationTexture)
}

unsafe extern "system" fn GetRenderTargetData(
    _this: *mut IDirect3DDevice9,
    pRenderTarget: *mut IDirect3DSurface9,
    pDestSurface: *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).GetRenderTargetData(pRenderTarget, pDestSurface)
}

unsafe extern "system" fn GetFrontBufferData(
    _this: *mut IDirect3DDevice9,
    iSwapChain: UINT,
    pDestSurface: *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).GetFrontBufferData(iSwapChain, pDestSurface)
}

unsafe extern "system" fn StretchRect(
    _this: *mut IDirect3DDevice9,
    pSourceSurface: *mut IDirect3DSurface9,
    pSourceRect: *const RECT,
    pDestSurface: *mut IDirect3DSurface9,
    pDestRect: *const RECT,
    Filter: D3DTEXTUREFILTERTYPE,
) -> HRESULT {
    (*D3D9_DEVICE).StretchRect(pSourceSurface, pSourceRect, pDestSurface, pDestRect, Filter)
}

unsafe extern "system" fn ColorFill(
    _this: *mut IDirect3DDevice9,
    pSurface: *mut IDirect3DSurface9,
    pRect: *const RECT,
    color: D3DCOLOR,
) -> HRESULT {
    (*D3D9_DEVICE).ColorFill(pSurface, pRect, color)
}

unsafe extern "system" fn CreateOffscreenPlainSurface(
    _this: *mut IDirect3DDevice9,
    Width: UINT,
    Height: UINT,
    Format: D3DFORMAT,
    Pool: D3DPOOL,
    ppSurface: *mut *mut IDirect3DSurface9,
    pSharedHandle: *mut HANDLE,
) -> HRESULT {
    (*D3D9_DEVICE).CreateOffscreenPlainSurface(
        Width,
        Height,
        Format,
        Pool,
        ppSurface,
        pSharedHandle,
    )
}

unsafe extern "system" fn SetRenderTarget(
    _this: *mut IDirect3DDevice9,
    RenderTargetIndex: DWORD,
    pRenderTarget: *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).SetRenderTarget(RenderTargetIndex, pRenderTarget)
}

unsafe extern "system" fn GetRenderTarget(
    _this: *mut IDirect3DDevice9,
    RenderTargetIndex: DWORD,
    ppRenderTarget: *mut *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).GetRenderTarget(RenderTargetIndex, ppRenderTarget)
}

unsafe extern "system" fn SetDepthStencilSurface(
    _this: *mut IDirect3DDevice9,
    pNewZStencil: *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).SetDepthStencilSurface(pNewZStencil)
}

unsafe extern "system" fn GetDepthStencilSurface(
    _this: *mut IDirect3DDevice9,
    ppZStencilSurface: *mut *mut IDirect3DSurface9,
) -> HRESULT {
    (*D3D9_DEVICE).GetDepthStencilSurface(ppZStencilSurface)
}

unsafe extern "system" fn BeginScene(_this: *mut IDirect3DDevice9) -> HRESULT {
    (*D3D9_DEVICE).BeginScene()
}

unsafe extern "system" fn EndScene(_this: *mut IDirect3DDevice9) -> HRESULT {
    if let Some(func) = RENDER_HOOK_FN {
        func(device());
    }

    (*D3D9_DEVICE).EndScene()
}

unsafe extern "system" fn Clear(
    _this: *mut IDirect3DDevice9,
    Count: DWORD,
    pRects: *const D3DRECT,
    Flags: DWORD,
    Color: D3DCOLOR,
    Z: FLOAT,
    Stencil: DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).Clear(Count, pRects, Flags, Color, Z, Stencil)
}

unsafe extern "system" fn SetTransform(
    _this: *mut IDirect3DDevice9,
    State: D3DTRANSFORMSTATETYPE,
    pMatrix: *const D3DMATRIX,
) -> HRESULT {
    (*D3D9_DEVICE).SetTransform(State, pMatrix)
}

unsafe extern "system" fn GetTransform(
    _this: *mut IDirect3DDevice9,
    State: D3DTRANSFORMSTATETYPE,
    pMatrix: *mut D3DMATRIX,
) -> HRESULT {
    (*D3D9_DEVICE).GetTransform(State, pMatrix)
}

unsafe extern "system" fn MultiplyTransform(
    _this: *mut IDirect3DDevice9,
    arg1: D3DTRANSFORMSTATETYPE,
    arg2: *const D3DMATRIX,
) -> HRESULT {
    (*D3D9_DEVICE).MultiplyTransform(arg1, arg2)
}

unsafe extern "system" fn SetViewport(
    _this: *mut IDirect3DDevice9,
    pViewport: *const D3DVIEWPORT9,
) -> HRESULT {
    (*D3D9_DEVICE).SetViewport(pViewport)
}

unsafe extern "system" fn GetViewport(
    _this: *mut IDirect3DDevice9,
    pViewport: *mut D3DVIEWPORT9,
) -> HRESULT {
    (*D3D9_DEVICE).GetViewport(pViewport)
}

unsafe extern "system" fn SetMaterial(
    _this: *mut IDirect3DDevice9,
    pMaterial: *const D3DMATERIAL9,
) -> HRESULT {
    (*D3D9_DEVICE).SetMaterial(pMaterial)
}

unsafe extern "system" fn GetMaterial(
    _this: *mut IDirect3DDevice9,
    pMaterial: *mut D3DMATERIAL9,
) -> HRESULT {
    (*D3D9_DEVICE).GetMaterial(pMaterial)
}

unsafe extern "system" fn SetLight(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    arg1: *const D3DLIGHT9,
) -> HRESULT {
    (*D3D9_DEVICE).SetLight(Index, arg1)
}

unsafe extern "system" fn GetLight(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    arg1: *mut D3DLIGHT9,
) -> HRESULT {
    (*D3D9_DEVICE).GetLight(Index, arg1)
}

unsafe extern "system" fn LightEnable(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    Enable: BOOL,
) -> HRESULT {
    (*D3D9_DEVICE).LightEnable(Index, Enable)
}

unsafe extern "system" fn GetLightEnable(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    pEnable: *mut BOOL,
) -> HRESULT {
    (*D3D9_DEVICE).GetLightEnable(Index, pEnable)
}

unsafe extern "system" fn SetClipPlane(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    pPlane: *const FLOAT,
) -> HRESULT {
    (*D3D9_DEVICE).SetClipPlane(Index, pPlane)
}

unsafe extern "system" fn GetClipPlane(
    _this: *mut IDirect3DDevice9,
    Index: DWORD,
    pPlane: *mut FLOAT,
) -> HRESULT {
    (*D3D9_DEVICE).GetClipPlane(Index, pPlane)
}

unsafe extern "system" fn SetRenderState(
    _this: *mut IDirect3DDevice9,
    State: D3DRENDERSTATETYPE,
    Value: DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).SetRenderState(State, Value)
}

unsafe extern "system" fn GetRenderState(
    _this: *mut IDirect3DDevice9,
    State: D3DRENDERSTATETYPE,
    pValue: *mut DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).GetRenderState(State, pValue)
}

unsafe extern "system" fn CreateStateBlock(
    _this: *mut IDirect3DDevice9,
    Type: D3DSTATEBLOCKTYPE,
    ppSB: *mut *mut IDirect3DStateBlock9,
) -> HRESULT {
    (*D3D9_DEVICE).CreateStateBlock(Type, ppSB)
}

unsafe extern "system" fn BeginStateBlock(_this: *mut IDirect3DDevice9) -> HRESULT {
    (*D3D9_DEVICE).BeginStateBlock()
}

unsafe extern "system" fn EndStateBlock(
    _this: *mut IDirect3DDevice9,
    ppSB: *mut *mut IDirect3DStateBlock9,
) -> HRESULT {
    (*D3D9_DEVICE).EndStateBlock(ppSB)
}

unsafe extern "system" fn SetClipStatus(
    _this: *mut IDirect3DDevice9,
    pClipStatus: *const D3DCLIPSTATUS9,
) -> HRESULT {
    (*D3D9_DEVICE).SetClipStatus(pClipStatus)
}

unsafe extern "system" fn GetClipStatus(
    _this: *mut IDirect3DDevice9,
    pClipStatus: *mut D3DCLIPSTATUS9,
) -> HRESULT {
    (*D3D9_DEVICE).GetClipStatus(pClipStatus)
}

unsafe extern "system" fn GetTexture(
    _this: *mut IDirect3DDevice9,
    Stage: DWORD,
    ppTexture: *mut *mut IDirect3DBaseTexture9,
) -> HRESULT {
    (*D3D9_DEVICE).GetTexture(Stage, ppTexture)
}

unsafe extern "system" fn SetTexture(
    _this: *mut IDirect3DDevice9,
    Stage: DWORD,
    pTexture: *mut IDirect3DBaseTexture9,
) -> HRESULT {
    (*D3D9_DEVICE).SetTexture(Stage, pTexture)
}

unsafe extern "system" fn GetTextureStageState(
    _this: *mut IDirect3DDevice9,
    Stage: DWORD,
    Type: D3DTEXTURESTAGESTATETYPE,
    pValue: *mut DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).GetTextureStageState(Stage, Type, pValue)
}

unsafe extern "system" fn SetTextureStageState(
    _this: *mut IDirect3DDevice9,
    Stage: DWORD,
    Type: D3DTEXTURESTAGESTATETYPE,
    Value: DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).SetTextureStageState(Stage, Type, Value)
}

unsafe extern "system" fn GetSamplerState(
    _this: *mut IDirect3DDevice9,
    Sampler: DWORD,
    Type: D3DSAMPLERSTATETYPE,
    pValue: *mut DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).GetSamplerState(Sampler, Type, pValue)
}

unsafe extern "system" fn SetSamplerState(
    _this: *mut IDirect3DDevice9,
    Sampler: DWORD,
    Type: D3DSAMPLERSTATETYPE,
    Value: DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).SetSamplerState(Sampler, Type, Value)
}

unsafe extern "system" fn ValidateDevice(
    _this: *mut IDirect3DDevice9,
    pNumPasses: *mut DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).ValidateDevice(pNumPasses)
}

unsafe extern "system" fn SetPaletteEntries(
    _this: *mut IDirect3DDevice9,
    PaletteNumber: UINT,
    pEntries: *const PALETTEENTRY,
) -> HRESULT {
    (*D3D9_DEVICE).SetPaletteEntries(PaletteNumber, pEntries)
}

unsafe extern "system" fn GetPaletteEntries(
    _this: *mut IDirect3DDevice9,
    PaletteNumber: UINT,
    pEntries: *mut PALETTEENTRY,
) -> HRESULT {
    (*D3D9_DEVICE).GetPaletteEntries(PaletteNumber, pEntries)
}

unsafe extern "system" fn SetCurrentTexturePalette(
    _this: *mut IDirect3DDevice9,
    PaletteNumber: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetCurrentTexturePalette(PaletteNumber)
}

unsafe extern "system" fn GetCurrentTexturePalette(
    _this: *mut IDirect3DDevice9,
    PaletteNumber: *mut UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetCurrentTexturePalette(PaletteNumber)
}

unsafe extern "system" fn SetScissorRect(
    _this: *mut IDirect3DDevice9,
    pRect: *const RECT,
) -> HRESULT {
    (*D3D9_DEVICE).SetScissorRect(pRect)
}

unsafe extern "system" fn GetScissorRect(
    _this: *mut IDirect3DDevice9,
    pRect: *mut RECT,
) -> HRESULT {
    (*D3D9_DEVICE).GetScissorRect(pRect)
}

unsafe extern "system" fn SetSoftwareVertexProcessing(
    _this: *mut IDirect3DDevice9,
    bSoftware: BOOL,
) -> HRESULT {
    (*D3D9_DEVICE).SetSoftwareVertexProcessing(bSoftware)
}

unsafe extern "system" fn GetSoftwareVertexProcessing(_this: *mut IDirect3DDevice9) -> BOOL {
    (*D3D9_DEVICE).GetSoftwareVertexProcessing()
}

unsafe extern "system" fn SetNPatchMode(_this: *mut IDirect3DDevice9, nSegments: FLOAT) -> HRESULT {
    (*D3D9_DEVICE).SetNPatchMode(nSegments)
}

unsafe extern "system" fn GetNPatchMode(_this: *mut IDirect3DDevice9) -> FLOAT {
    (*D3D9_DEVICE).GetNPatchMode()
}

unsafe extern "system" fn DrawPrimitive(
    _this: *mut IDirect3DDevice9,
    PrimitiveType: D3DPRIMITIVETYPE,
    StartVertex: UINT,
    PrimitiveCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).DrawPrimitive(PrimitiveType, StartVertex, PrimitiveCount)
}

unsafe extern "system" fn DrawIndexedPrimitive(
    _this: *mut IDirect3DDevice9,
    arg1: D3DPRIMITIVETYPE,
    BaseVertexIndex: INT,
    MinVertexIndex: UINT,
    NumVertices: UINT,
    startIndex: UINT,
    primCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).DrawIndexedPrimitive(
        arg1,
        BaseVertexIndex,
        MinVertexIndex,
        NumVertices,
        startIndex,
        primCount,
    )
}

unsafe extern "system" fn DrawPrimitiveUP(
    _this: *mut IDirect3DDevice9,
    PrimitiveType: D3DPRIMITIVETYPE,
    PrimitiveCount: UINT,
    pVertexStreamZeroData: *const VOID,
    VertexStreamZeroStride: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).DrawPrimitiveUP(
        PrimitiveType,
        PrimitiveCount,
        pVertexStreamZeroData,
        VertexStreamZeroStride,
    )
}

unsafe extern "system" fn DrawIndexedPrimitiveUP(
    _this: *mut IDirect3DDevice9,
    PrimitiveType: D3DPRIMITIVETYPE,
    MinVertexIndex: UINT,
    NumVertices: UINT,
    PrimitiveCount: UINT,
    pIndexData: *const VOID,
    IndexDataFormat: D3DFORMAT,
    pVertexStreamZeroData: *const VOID,
    VertexStreamZeroStride: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).DrawIndexedPrimitiveUP(
        PrimitiveType,
        MinVertexIndex,
        NumVertices,
        PrimitiveCount,
        pIndexData,
        IndexDataFormat,
        pVertexStreamZeroData,
        VertexStreamZeroStride,
    )
}

unsafe extern "system" fn ProcessVertices(
    _this: *mut IDirect3DDevice9,
    SrcStartIndex: UINT,
    DestIndex: UINT,
    VertexCount: UINT,
    pDestBuffer: *mut IDirect3DVertexBuffer9,
    pVertexDecl: *mut IDirect3DVertexDeclaration9,
    Flags: DWORD,
) -> HRESULT {
    (*D3D9_DEVICE).ProcessVertices(
        SrcStartIndex,
        DestIndex,
        VertexCount,
        pDestBuffer,
        pVertexDecl,
        Flags,
    )
}

unsafe extern "system" fn CreateVertexDeclaration(
    _this: *mut IDirect3DDevice9,
    pVertexElements: *const D3DVERTEXELEMENT9,
    ppDecl: *mut *mut IDirect3DVertexDeclaration9,
) -> HRESULT {
    (*D3D9_DEVICE).CreateVertexDeclaration(pVertexElements, ppDecl)
}

unsafe extern "system" fn SetVertexDeclaration(
    _this: *mut IDirect3DDevice9,
    pDecl: *mut IDirect3DVertexDeclaration9,
) -> HRESULT {
    (*D3D9_DEVICE).SetVertexDeclaration(pDecl)
}

unsafe extern "system" fn GetVertexDeclaration(
    _this: *mut IDirect3DDevice9,
    ppDecl: *mut *mut IDirect3DVertexDeclaration9,
) -> HRESULT {
    (*D3D9_DEVICE).GetVertexDeclaration(ppDecl)
}

unsafe extern "system" fn SetFVF(_this: *mut IDirect3DDevice9, FVF: DWORD) -> HRESULT {
    (*D3D9_DEVICE).SetFVF(FVF)
}

unsafe extern "system" fn GetFVF(_this: *mut IDirect3DDevice9, pFVF: *mut DWORD) -> HRESULT {
    (*D3D9_DEVICE).GetFVF(pFVF)
}

unsafe extern "system" fn CreateVertexShader(
    _this: *mut IDirect3DDevice9,
    pFunction: *const DWORD,
    ppShader: *mut *mut IDirect3DVertexShader9,
) -> HRESULT {
    (*D3D9_DEVICE).CreateVertexShader(pFunction, ppShader)
}

unsafe extern "system" fn SetVertexShader(
    _this: *mut IDirect3DDevice9,
    pShader: *mut IDirect3DVertexShader9,
) -> HRESULT {
    (*D3D9_DEVICE).SetVertexShader(pShader)
}

unsafe extern "system" fn GetVertexShader(
    _this: *mut IDirect3DDevice9,
    ppShader: *mut *mut IDirect3DVertexShader9,
) -> HRESULT {
    (*D3D9_DEVICE).GetVertexShader(ppShader)
}

unsafe extern "system" fn SetVertexShaderConstantF(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const FLOAT,
    Vector4fCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetVertexShaderConstantF(StartRegister, pConstantData, Vector4fCount)
}

unsafe extern "system" fn GetVertexShaderConstantF(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut FLOAT,
    Vector4fCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetVertexShaderConstantF(StartRegister, pConstantData, Vector4fCount)
}

unsafe extern "system" fn SetVertexShaderConstantI(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const INT,
    Vector4iCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetVertexShaderConstantI(StartRegister, pConstantData, Vector4iCount)
}

unsafe extern "system" fn GetVertexShaderConstantI(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut INT,
    Vector4iCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetVertexShaderConstantI(StartRegister, pConstantData, Vector4iCount)
}

unsafe extern "system" fn SetVertexShaderConstantB(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const BOOL,
    BoolCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetVertexShaderConstantB(StartRegister, pConstantData, BoolCount)
}

unsafe extern "system" fn GetVertexShaderConstantB(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut BOOL,
    BoolCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetVertexShaderConstantB(StartRegister, pConstantData, BoolCount)
}

unsafe extern "system" fn SetStreamSource(
    _this: *mut IDirect3DDevice9,
    StreamNumber: UINT,
    pStreamData: *mut IDirect3DVertexBuffer9,
    OffsetInBytes: UINT,
    Stride: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetStreamSource(StreamNumber, pStreamData, OffsetInBytes, Stride)
}

unsafe extern "system" fn GetStreamSource(
    _this: *mut IDirect3DDevice9,
    StreamNumber: UINT,
    ppStreamData: *mut *mut IDirect3DVertexBuffer9,
    pOffsetInBytes: *mut UINT,
    pStride: *mut UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetStreamSource(StreamNumber, ppStreamData, pOffsetInBytes, pStride)
}

unsafe extern "system" fn SetStreamSourceFreq(
    _this: *mut IDirect3DDevice9,
    StreamNumber: UINT,
    Setting: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetStreamSourceFreq(StreamNumber, Setting)
}

unsafe extern "system" fn GetStreamSourceFreq(
    _this: *mut IDirect3DDevice9,
    StreamNumber: UINT,
    pSetting: *mut UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetStreamSourceFreq(StreamNumber, pSetting)
}

unsafe extern "system" fn SetIndices(
    _this: *mut IDirect3DDevice9,
    pIndexData: *mut IDirect3DIndexBuffer9,
) -> HRESULT {
    (*D3D9_DEVICE).SetIndices(pIndexData)
}

unsafe extern "system" fn GetIndices(
    _this: *mut IDirect3DDevice9,
    ppIndexData: *mut *mut IDirect3DIndexBuffer9,
) -> HRESULT {
    (*D3D9_DEVICE).GetIndices(ppIndexData)
}

unsafe extern "system" fn CreatePixelShader(
    _this: *mut IDirect3DDevice9,
    pFunction: *const DWORD,
    ppShader: *mut *mut IDirect3DPixelShader9,
) -> HRESULT {
    (*D3D9_DEVICE).CreatePixelShader(pFunction, ppShader)
}

unsafe extern "system" fn SetPixelShader(
    _this: *mut IDirect3DDevice9,
    pShader: *mut IDirect3DPixelShader9,
) -> HRESULT {
    (*D3D9_DEVICE).SetPixelShader(pShader)
}

unsafe extern "system" fn GetPixelShader(
    _this: *mut IDirect3DDevice9,
    ppShader: *mut *mut IDirect3DPixelShader9,
) -> HRESULT {
    (*D3D9_DEVICE).GetPixelShader(ppShader)
}

unsafe extern "system" fn SetPixelShaderConstantF(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const FLOAT,
    Vector4fCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetPixelShaderConstantF(StartRegister, pConstantData, Vector4fCount)
}

unsafe extern "system" fn GetPixelShaderConstantF(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut FLOAT,
    Vector4fCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetPixelShaderConstantF(StartRegister, pConstantData, Vector4fCount)
}

unsafe extern "system" fn SetPixelShaderConstantI(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const INT,
    Vector4iCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetPixelShaderConstantI(StartRegister, pConstantData, Vector4iCount)
}

unsafe extern "system" fn GetPixelShaderConstantI(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut INT,
    Vector4iCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetPixelShaderConstantI(StartRegister, pConstantData, Vector4iCount)
}

unsafe extern "system" fn SetPixelShaderConstantB(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *const BOOL,
    BoolCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).SetPixelShaderConstantB(StartRegister, pConstantData, BoolCount)
}

unsafe extern "system" fn GetPixelShaderConstantB(
    _this: *mut IDirect3DDevice9,
    StartRegister: UINT,
    pConstantData: *mut BOOL,
    BoolCount: UINT,
) -> HRESULT {
    (*D3D9_DEVICE).GetPixelShaderConstantB(StartRegister, pConstantData, BoolCount)
}

unsafe extern "system" fn DrawRectPatch(
    _this: *mut IDirect3DDevice9,
    Handle: UINT,
    pNumSegs: *const FLOAT,
    pRectPatchInfo: *const D3DRECTPATCH_INFO,
) -> HRESULT {
    (*D3D9_DEVICE).DrawRectPatch(Handle, pNumSegs, pRectPatchInfo)
}

unsafe extern "system" fn DrawTriPatch(
    _this: *mut IDirect3DDevice9,
    Handle: UINT,
    pNumSegs: *const FLOAT,
    pTriPatchInfo: *const D3DTRIPATCH_INFO,
) -> HRESULT {
    (*D3D9_DEVICE).DrawTriPatch(Handle, pNumSegs, pTriPatchInfo)
}

unsafe extern "system" fn DeletePatch(_this: *mut IDirect3DDevice9, Handle: UINT) -> HRESULT {
    (*D3D9_DEVICE).DeletePatch(Handle)
}

unsafe extern "system" fn CreateQuery(
    _this: *mut IDirect3DDevice9,
    Type: D3DQUERYTYPE,
    ppQuery: *mut *mut IDirect3DQuery9,
) -> HRESULT {
    (*D3D9_DEVICE).CreateQuery(Type, ppQuery)
}
