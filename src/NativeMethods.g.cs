// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace CuteLights.Sdk
{
    internal static unsafe partial class NativeMethods
    {
        const string __DllName = "libcutelight.so";



        [DllImport(__DllName, EntryPoint = "light_set_on", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static extern bool light_set_on(LightPtr* l, [MarshalAs(UnmanagedType.U1)] bool on);

        [DllImport(__DllName, EntryPoint = "light_set_color", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static extern bool light_set_color(LightPtr* l, byte red, byte green, byte blue);

        [DllImport(__DllName, EntryPoint = "light_set_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static extern bool light_set_brightness(LightPtr* l, byte brightness);

        [DllImport(__DllName, EntryPoint = "light_get_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte light_get_brightness(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_red", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte light_get_red(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_green", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte light_get_green(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_blue", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte light_get_blue(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_is_on", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static extern bool light_get_is_on(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_name", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte* light_get_name(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_id", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern byte* light_get_id(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_get_supports_color", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        internal static extern bool light_get_supports_color(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void light_free(LightPtr* l);

        [DllImport(__DllName, EntryPoint = "light_discoverer_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern LightDiscovererPtr* light_discoverer_new();

        [DllImport(__DllName, EntryPoint = "light_discoverer_next", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern LightPtr* light_discoverer_next(LightDiscovererPtr* ld);

        [DllImport(__DllName, EntryPoint = "light_discoverer_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void light_discoverer_free(LightDiscovererPtr* ld);

        [DllImport(__DllName, EntryPoint = "frame_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern FramePtr* frame_new();

        [DllImport(__DllName, EntryPoint = "frame_clear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_clear(FramePtr* f);

        [DllImport(__DllName, EntryPoint = "frame_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_free(FramePtr* f);

        [DllImport(__DllName, EntryPoint = "frame_set_on", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_set_on(FramePtr* f, LightPtr* l, [MarshalAs(UnmanagedType.U1)] bool on);

        [DllImport(__DllName, EntryPoint = "frame_set_color", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_set_color(FramePtr* f, LightPtr* l, byte red, byte green, byte blue);

        [DllImport(__DllName, EntryPoint = "frame_set_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_set_brightness(FramePtr* f, LightPtr* l, byte brightness);

        [DllImport(__DllName, EntryPoint = "frame_run", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void frame_run(FramePtr* f);


    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct FramePtr
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct LightDiscovererPtr
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct LightPtr
    {
        public fixed byte _unused[1];
    }



}
