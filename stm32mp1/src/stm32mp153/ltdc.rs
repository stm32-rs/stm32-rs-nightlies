#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ltdc_idr: LTDC_IDR,
    ltdc_lcr: LTDC_LCR,
    ltdc_sscr: LTDC_SSCR,
    ltdc_bpcr: LTDC_BPCR,
    ltdc_awcr: LTDC_AWCR,
    ltdc_twcr: LTDC_TWCR,
    ltdc_gcr: LTDC_GCR,
    ltdc_gc1r: LTDC_GC1R,
    ltdc_gc2r: LTDC_GC2R,
    ltdc_srcr: LTDC_SRCR,
    _reserved10: [u8; 0x04],
    ltdc_bccr: LTDC_BCCR,
    _reserved11: [u8; 0x04],
    ltdc_ier: LTDC_IER,
    ltdc_isr: LTDC_ISR,
    ltdc_icr: LTDC_ICR,
    ltdc_lipcr: LTDC_LIPCR,
    ltdc_cpsr: LTDC_CPSR,
    ltdc_cdsr: LTDC_CDSR,
    _reserved17: [u8; 0x38],
    ltdc_l1cr: LTDC_L1CR,
    ltdc_l1whpcr: LTDC_L1WHPCR,
    ltdc_l1wvpcr: LTDC_L1WVPCR,
    ltdc_l1ckcr: LTDC_L1CKCR,
    ltdc_l1pfcr: LTDC_L1PFCR,
    ltdc_l1cacr: LTDC_L1CACR,
    ltdc_l1dccr: LTDC_L1DCCR,
    ltdc_l1bfcr: LTDC_L1BFCR,
    _reserved25: [u8; 0x08],
    ltdc_l1cfbar: LTDC_L1CFBAR,
    ltdc_l1cfblr: LTDC_L1CFBLR,
    ltdc_l1cfblnr: LTDC_L1CFBLNR,
    _reserved28: [u8; 0x0c],
    ltdc_l1clutwr: LTDC_L1CLUTWR,
    _reserved29: [u8; 0x3c],
    ltdc_l2cr: LTDC_L2CR,
    ltdc_l2whpcr: LTDC_L2WHPCR,
    ltdc_l2wvpcr: LTDC_L2WVPCR,
    ltdc_l2ckcr: LTDC_L2CKCR,
    ltdc_l2pfcr: LTDC_L2PFCR,
    ltdc_l2cacr: LTDC_L2CACR,
    ltdc_l2dccr: LTDC_L2DCCR,
    ltdc_l2bfcr: LTDC_L2BFCR,
    _reserved37: [u8; 0x08],
    ltdc_l2cfbar: LTDC_L2CFBAR,
    ltdc_l2cfblr: LTDC_L2CFBLR,
    ltdc_l2cfblnr: LTDC_L2CFBLNR,
    _reserved40: [u8; 0x0c],
    ltdc_l2clutwr: LTDC_L2CLUTWR,
}
impl RegisterBlock {
    ///0x00 - LTDC identification register
    #[inline(always)]
    pub const fn ltdc_idr(&self) -> &LTDC_IDR {
        &self.ltdc_idr
    }
    ///0x04 - LDTC layer count register
    #[inline(always)]
    pub const fn ltdc_lcr(&self) -> &LTDC_LCR {
        &self.ltdc_lcr
    }
    ///0x08 - This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn ltdc_sscr(&self) -> &LTDC_SSCR {
        &self.ltdc_sscr
    }
    ///0x0c - This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn ltdc_bpcr(&self) -> &LTDC_BPCR {
        &self.ltdc_bpcr
    }
    ///0x10 - This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn ltdc_awcr(&self) -> &LTDC_AWCR {
        &self.ltdc_awcr
    }
    ///0x14 - This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn ltdc_twcr(&self) -> &LTDC_TWCR {
        &self.ltdc_twcr
    }
    ///0x18 - This register defines the global configuration of the LCD-TFT controller.
    #[inline(always)]
    pub const fn ltdc_gcr(&self) -> &LTDC_GCR {
        &self.ltdc_gcr
    }
    ///0x1c - LTDC global configuration 1 register
    #[inline(always)]
    pub const fn ltdc_gc1r(&self) -> &LTDC_GC1R {
        &self.ltdc_gc1r
    }
    ///0x20 - LTDC global configuration 2 register
    #[inline(always)]
    pub const fn ltdc_gc2r(&self) -> &LTDC_GC2R {
        &self.ltdc_gc2r
    }
    ///0x24 - This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
    #[inline(always)]
    pub const fn ltdc_srcr(&self) -> &LTDC_SRCR {
        &self.ltdc_srcr
    }
    ///0x2c - This register defines the background color (RGB888).
    #[inline(always)]
    pub const fn ltdc_bccr(&self) -> &LTDC_BCCR {
        &self.ltdc_bccr
    }
    ///0x34 - This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    #[inline(always)]
    pub const fn ltdc_ier(&self) -> &LTDC_IER {
        &self.ltdc_ier
    }
    ///0x38 - This register returns the interrupt status flag.
    #[inline(always)]
    pub const fn ltdc_isr(&self) -> &LTDC_ISR {
        &self.ltdc_isr
    }
    ///0x3c - LTDC Interrupt Clear Register
    #[inline(always)]
    pub const fn ltdc_icr(&self) -> &LTDC_ICR {
        &self.ltdc_icr
    }
    ///0x40 - This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
    #[inline(always)]
    pub const fn ltdc_lipcr(&self) -> &LTDC_LIPCR {
        &self.ltdc_lipcr
    }
    ///0x44 - LTDC current position status register
    #[inline(always)]
    pub const fn ltdc_cpsr(&self) -> &LTDC_CPSR {
        &self.ltdc_cpsr
    }
    ///0x48 - This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
    #[inline(always)]
    pub const fn ltdc_cdsr(&self) -> &LTDC_CDSR {
        &self.ltdc_cdsr
    }
    ///0x84 - LTDC layer 1 control register
    #[inline(always)]
    pub const fn ltdc_l1cr(&self) -> &LTDC_L1CR {
        &self.ltdc_l1cr
    }
    /**0x88 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
    bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
    bits in the LTDC_AWCR register.*/
    #[inline(always)]
    pub const fn ltdc_l1whpcr(&self) -> &LTDC_L1WHPCR {
        &self.ltdc_l1whpcr
    }
    /**0x8c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
    bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
    bits in the LTDC_AWCR register.*/
    #[inline(always)]
    pub const fn ltdc_l1wvpcr(&self) -> &LTDC_L1WVPCR {
        &self.ltdc_l1wvpcr
    }
    ///0x90 - This register defines the color key value (RGB), that is used by the color keying.
    #[inline(always)]
    pub const fn ltdc_l1ckcr(&self) -> &LTDC_L1CKCR {
        &self.ltdc_l1ckcr
    }
    ///0x94 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    #[inline(always)]
    pub const fn ltdc_l1pfcr(&self) -> &LTDC_L1PFCR {
        &self.ltdc_l1pfcr
    }
    ///0x98 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    #[inline(always)]
    pub const fn ltdc_l1cacr(&self) -> &LTDC_L1CACR {
        &self.ltdc_l1cacr
    }
    ///0x9c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    #[inline(always)]
    pub const fn ltdc_l1dccr(&self) -> &LTDC_L1DCCR {
        &self.ltdc_l1dccr
    }
    ///0xa0 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    #[inline(always)]
    pub const fn ltdc_l1bfcr(&self) -> &LTDC_L1BFCR {
        &self.ltdc_l1bfcr
    }
    ///0xac - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    #[inline(always)]
    pub const fn ltdc_l1cfbar(&self) -> &LTDC_L1CFBAR {
        &self.ltdc_l1cfbar
    }
    ///0xb0 - This register defines the color frame buffer line length and pitch.
    #[inline(always)]
    pub const fn ltdc_l1cfblr(&self) -> &LTDC_L1CFBLR {
        &self.ltdc_l1cfblr
    }
    ///0xb4 - This register defines the number of lines in the color frame buffer.
    #[inline(always)]
    pub const fn ltdc_l1cfblnr(&self) -> &LTDC_L1CFBLNR {
        &self.ltdc_l1cfblnr
    }
    ///0xc4 - This register defines the CLUT address and the RGB value.
    #[inline(always)]
    pub const fn ltdc_l1clutwr(&self) -> &LTDC_L1CLUTWR {
        &self.ltdc_l1clutwr
    }
    ///0x104 - LTDC layer 2 control register
    #[inline(always)]
    pub const fn ltdc_l2cr(&self) -> &LTDC_L2CR {
        &self.ltdc_l2cr
    }
    /**0x108 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
    bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
    bits in the LTDC_AWCR register.*/
    #[inline(always)]
    pub const fn ltdc_l2whpcr(&self) -> &LTDC_L2WHPCR {
        &self.ltdc_l2whpcr
    }
    /**0x10c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
    bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
    bits in the LTDC_AWCR register.*/
    #[inline(always)]
    pub const fn ltdc_l2wvpcr(&self) -> &LTDC_L2WVPCR {
        &self.ltdc_l2wvpcr
    }
    ///0x110 - This register defines the color key value (RGB), that is used by the color keying.
    #[inline(always)]
    pub const fn ltdc_l2ckcr(&self) -> &LTDC_L2CKCR {
        &self.ltdc_l2ckcr
    }
    ///0x114 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    #[inline(always)]
    pub const fn ltdc_l2pfcr(&self) -> &LTDC_L2PFCR {
        &self.ltdc_l2pfcr
    }
    ///0x118 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    #[inline(always)]
    pub const fn ltdc_l2cacr(&self) -> &LTDC_L2CACR {
        &self.ltdc_l2cacr
    }
    ///0x11c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    #[inline(always)]
    pub const fn ltdc_l2dccr(&self) -> &LTDC_L2DCCR {
        &self.ltdc_l2dccr
    }
    ///0x120 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    #[inline(always)]
    pub const fn ltdc_l2bfcr(&self) -> &LTDC_L2BFCR {
        &self.ltdc_l2bfcr
    }
    ///0x12c - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    #[inline(always)]
    pub const fn ltdc_l2cfbar(&self) -> &LTDC_L2CFBAR {
        &self.ltdc_l2cfbar
    }
    ///0x130 - This register defines the color frame buffer line length and pitch.
    #[inline(always)]
    pub const fn ltdc_l2cfblr(&self) -> &LTDC_L2CFBLR {
        &self.ltdc_l2cfblr
    }
    ///0x134 - This register defines the number of lines in the color frame buffer.
    #[inline(always)]
    pub const fn ltdc_l2cfblnr(&self) -> &LTDC_L2CFBLNR {
        &self.ltdc_l2cfblnr
    }
    ///0x144 - This register defines the CLUT address and the RGB value.
    #[inline(always)]
    pub const fn ltdc_l2clutwr(&self) -> &LTDC_L2CLUTWR {
        &self.ltdc_l2clutwr
    }
}
/**LTDC_IDR (r) register accessor: LTDC identification register

You can [`read`](crate::Reg::read) this register and get [`ltdc_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_IDR)

For information about available fields see [`mod@ltdc_idr`]
module*/
pub type LTDC_IDR = crate::Reg<ltdc_idr::LTDC_IDRrs>;
///LTDC identification register
pub mod ltdc_idr;
/**LTDC_LCR (r) register accessor: LDTC layer count register

You can [`read`](crate::Reg::read) this register and get [`ltdc_lcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_LCR)

For information about available fields see [`mod@ltdc_lcr`]
module*/
pub type LTDC_LCR = crate::Reg<ltdc_lcr::LTDC_LCRrs>;
///LDTC layer count register
pub mod ltdc_lcr;
/**LTDC_SSCR (rw) register accessor: This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`ltdc_sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_SSCR)

For information about available fields see [`mod@ltdc_sscr`]
module*/
pub type LTDC_SSCR = crate::Reg<ltdc_sscr::LTDC_SSCRrs>;
///This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_sscr;
/**LTDC_BPCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`ltdc_bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_BPCR)

For information about available fields see [`mod@ltdc_bpcr`]
module*/
pub type LTDC_BPCR = crate::Reg<ltdc_bpcr::LTDC_BPCRrs>;
///This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_bpcr;
/**LTDC_AWCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`ltdc_awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_AWCR)

For information about available fields see [`mod@ltdc_awcr`]
module*/
pub type LTDC_AWCR = crate::Reg<ltdc_awcr::LTDC_AWCRrs>;
///This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_awcr;
/**LTDC_TWCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`ltdc_twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_TWCR)

For information about available fields see [`mod@ltdc_twcr`]
module*/
pub type LTDC_TWCR = crate::Reg<ltdc_twcr::LTDC_TWCRrs>;
///This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod ltdc_twcr;
/**LTDC_GCR (rw) register accessor: This register defines the global configuration of the LCD-TFT controller.

You can [`read`](crate::Reg::read) this register and get [`ltdc_gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_GCR)

For information about available fields see [`mod@ltdc_gcr`]
module*/
pub type LTDC_GCR = crate::Reg<ltdc_gcr::LTDC_GCRrs>;
///This register defines the global configuration of the LCD-TFT controller.
pub mod ltdc_gcr;
/**LTDC_GC1R (r) register accessor: LTDC global configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`ltdc_gc1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_GC1R)

For information about available fields see [`mod@ltdc_gc1r`]
module*/
pub type LTDC_GC1R = crate::Reg<ltdc_gc1r::LTDC_GC1Rrs>;
///LTDC global configuration 1 register
pub mod ltdc_gc1r;
/**LTDC_GC2R (r) register accessor: LTDC global configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`ltdc_gc2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_GC2R)

For information about available fields see [`mod@ltdc_gc2r`]
module*/
pub type LTDC_GC2R = crate::Reg<ltdc_gc2r::LTDC_GC2Rrs>;
///LTDC global configuration 2 register
pub mod ltdc_gc2r;
/**LTDC_SRCR (rw) register accessor: This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.

You can [`read`](crate::Reg::read) this register and get [`ltdc_srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_SRCR)

For information about available fields see [`mod@ltdc_srcr`]
module*/
pub type LTDC_SRCR = crate::Reg<ltdc_srcr::LTDC_SRCRrs>;
///This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
pub mod ltdc_srcr;
/**LTDC_BCCR (rw) register accessor: This register defines the background color (RGB888).

You can [`read`](crate::Reg::read) this register and get [`ltdc_bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_BCCR)

For information about available fields see [`mod@ltdc_bccr`]
module*/
pub type LTDC_BCCR = crate::Reg<ltdc_bccr::LTDC_BCCRrs>;
///This register defines the background color (RGB888).
pub mod ltdc_bccr;
/**LTDC_IER (rw) register accessor: This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`ltdc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_IER)

For information about available fields see [`mod@ltdc_ier`]
module*/
pub type LTDC_IER = crate::Reg<ltdc_ier::LTDC_IERrs>;
///This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod ltdc_ier;
/**LTDC_ISR (r) register accessor: This register returns the interrupt status flag.

You can [`read`](crate::Reg::read) this register and get [`ltdc_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_ISR)

For information about available fields see [`mod@ltdc_isr`]
module*/
pub type LTDC_ISR = crate::Reg<ltdc_isr::LTDC_ISRrs>;
///This register returns the interrupt status flag.
pub mod ltdc_isr;
/**LTDC_ICR (w) register accessor: LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_ICR)

For information about available fields see [`mod@ltdc_icr`]
module*/
pub type LTDC_ICR = crate::Reg<ltdc_icr::LTDC_ICRrs>;
///LTDC Interrupt Clear Register
pub mod ltdc_icr;
/**LTDC_LIPCR (rw) register accessor: This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.

You can [`read`](crate::Reg::read) this register and get [`ltdc_lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_LIPCR)

For information about available fields see [`mod@ltdc_lipcr`]
module*/
pub type LTDC_LIPCR = crate::Reg<ltdc_lipcr::LTDC_LIPCRrs>;
///This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
pub mod ltdc_lipcr;
/**LTDC_CPSR (r) register accessor: LTDC current position status register

You can [`read`](crate::Reg::read) this register and get [`ltdc_cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_CPSR)

For information about available fields see [`mod@ltdc_cpsr`]
module*/
pub type LTDC_CPSR = crate::Reg<ltdc_cpsr::LTDC_CPSRrs>;
///LTDC current position status register
pub mod ltdc_cpsr;
/**LTDC_CDSR (r) register accessor: This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.

You can [`read`](crate::Reg::read) this register and get [`ltdc_cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_CDSR)

For information about available fields see [`mod@ltdc_cdsr`]
module*/
pub type LTDC_CDSR = crate::Reg<ltdc_cdsr::LTDC_CDSRrs>;
///This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
pub mod ltdc_cdsr;
/**LTDC_L1CR (rw) register accessor: LTDC layer 1 control register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CR)

For information about available fields see [`mod@ltdc_l1cr`]
module*/
pub type LTDC_L1CR = crate::Reg<ltdc_l1cr::LTDC_L1CRrs>;
///LTDC layer 1 control register
pub mod ltdc_l1cr;
/**LTDC_L1WHPCR (rw) register accessor: This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1WHPCR)

For information about available fields see [`mod@ltdc_l1whpcr`]
module*/
pub type LTDC_L1WHPCR = crate::Reg<ltdc_l1whpcr::LTDC_L1WHPCRrs>;
/**This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
bits in the LTDC_AWCR register.*/
pub mod ltdc_l1whpcr;
/**LTDC_L1WVPCR (rw) register accessor: This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1WVPCR)

For information about available fields see [`mod@ltdc_l1wvpcr`]
module*/
pub type LTDC_L1WVPCR = crate::Reg<ltdc_l1wvpcr::LTDC_L1WVPCRrs>;
/**This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
bits in the LTDC_AWCR register.*/
pub mod ltdc_l1wvpcr;
/**LTDC_L1CKCR (rw) register accessor: This register defines the color key value (RGB), that is used by the color keying.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CKCR)

For information about available fields see [`mod@ltdc_l1ckcr`]
module*/
pub type LTDC_L1CKCR = crate::Reg<ltdc_l1ckcr::LTDC_L1CKCRrs>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod ltdc_l1ckcr;
/**LTDC_L1PFCR (rw) register accessor: This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1PFCR)

For information about available fields see [`mod@ltdc_l1pfcr`]
module*/
pub type LTDC_L1PFCR = crate::Reg<ltdc_l1pfcr::LTDC_L1PFCRrs>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod ltdc_l1pfcr;
/**LTDC_L1CACR (rw) register accessor: This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CACR)

For information about available fields see [`mod@ltdc_l1cacr`]
module*/
pub type LTDC_L1CACR = crate::Reg<ltdc_l1cacr::LTDC_L1CACRrs>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod ltdc_l1cacr;
/**LTDC_L1DCCR (rw) register accessor: This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1DCCR)

For information about available fields see [`mod@ltdc_l1dccr`]
module*/
pub type LTDC_L1DCCR = crate::Reg<ltdc_l1dccr::LTDC_L1DCCRrs>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod ltdc_l1dccr;
/**LTDC_L1BFCR (rw) register accessor: This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1BFCR)

For information about available fields see [`mod@ltdc_l1bfcr`]
module*/
pub type LTDC_L1BFCR = crate::Reg<ltdc_l1bfcr::LTDC_L1BFCRrs>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod ltdc_l1bfcr;
/**LTDC_L1CFBAR (rw) register accessor: This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CFBAR)

For information about available fields see [`mod@ltdc_l1cfbar`]
module*/
pub type LTDC_L1CFBAR = crate::Reg<ltdc_l1cfbar::LTDC_L1CFBARrs>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod ltdc_l1cfbar;
/**LTDC_L1CFBLR (rw) register accessor: This register defines the color frame buffer line length and pitch.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CFBLR)

For information about available fields see [`mod@ltdc_l1cfblr`]
module*/
pub type LTDC_L1CFBLR = crate::Reg<ltdc_l1cfblr::LTDC_L1CFBLRrs>;
///This register defines the color frame buffer line length and pitch.
pub mod ltdc_l1cfblr;
/**LTDC_L1CFBLNR (rw) register accessor: This register defines the number of lines in the color frame buffer.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CFBLNR)

For information about available fields see [`mod@ltdc_l1cfblnr`]
module*/
pub type LTDC_L1CFBLNR = crate::Reg<ltdc_l1cfblnr::LTDC_L1CFBLNRrs>;
///This register defines the number of lines in the color frame buffer.
pub mod ltdc_l1cfblnr;
/**LTDC_L1CLUTWR (w) register accessor: This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CLUTWR)

For information about available fields see [`mod@ltdc_l1clutwr`]
module*/
pub type LTDC_L1CLUTWR = crate::Reg<ltdc_l1clutwr::LTDC_L1CLUTWRrs>;
///This register defines the CLUT address and the RGB value.
pub mod ltdc_l1clutwr;
/**LTDC_L2CR (rw) register accessor: LTDC layer 2 control register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CR)

For information about available fields see [`mod@ltdc_l2cr`]
module*/
pub type LTDC_L2CR = crate::Reg<ltdc_l2cr::LTDC_L2CRrs>;
///LTDC layer 2 control register
pub mod ltdc_l2cr;
/**LTDC_L2WHPCR (rw) register accessor: This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2WHPCR)

For information about available fields see [`mod@ltdc_l2whpcr`]
module*/
pub type LTDC_L2WHPCR = crate::Reg<ltdc_l2whpcr::LTDC_L2WHPCRrs>;
/**This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
bits in the LTDC_AWCR register.*/
pub mod ltdc_l2whpcr;
/**LTDC_L2WVPCR (rw) register accessor: This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2WVPCR)

For information about available fields see [`mod@ltdc_l2wvpcr`]
module*/
pub type LTDC_L2WVPCR = crate::Reg<ltdc_l2wvpcr::LTDC_L2WVPCRrs>;
/**This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\]
bits in the LTDC_AWCR register.*/
pub mod ltdc_l2wvpcr;
/**LTDC_L2CKCR (rw) register accessor: This register defines the color key value (RGB), that is used by the color keying.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CKCR)

For information about available fields see [`mod@ltdc_l2ckcr`]
module*/
pub type LTDC_L2CKCR = crate::Reg<ltdc_l2ckcr::LTDC_L2CKCRrs>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod ltdc_l2ckcr;
/**LTDC_L2PFCR (rw) register accessor: This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2PFCR)

For information about available fields see [`mod@ltdc_l2pfcr`]
module*/
pub type LTDC_L2PFCR = crate::Reg<ltdc_l2pfcr::LTDC_L2PFCRrs>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod ltdc_l2pfcr;
/**LTDC_L2CACR (rw) register accessor: This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CACR)

For information about available fields see [`mod@ltdc_l2cacr`]
module*/
pub type LTDC_L2CACR = crate::Reg<ltdc_l2cacr::LTDC_L2CACRrs>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod ltdc_l2cacr;
/**LTDC_L2DCCR (rw) register accessor: This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2DCCR)

For information about available fields see [`mod@ltdc_l2dccr`]
module*/
pub type LTDC_L2DCCR = crate::Reg<ltdc_l2dccr::LTDC_L2DCCRrs>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod ltdc_l2dccr;
/**LTDC_L2BFCR (rw) register accessor: This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2BFCR)

For information about available fields see [`mod@ltdc_l2bfcr`]
module*/
pub type LTDC_L2BFCR = crate::Reg<ltdc_l2bfcr::LTDC_L2BFCRrs>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod ltdc_l2bfcr;
/**LTDC_L2CFBAR (rw) register accessor: This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CFBAR)

For information about available fields see [`mod@ltdc_l2cfbar`]
module*/
pub type LTDC_L2CFBAR = crate::Reg<ltdc_l2cfbar::LTDC_L2CFBARrs>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod ltdc_l2cfbar;
/**LTDC_L2CFBLR (rw) register accessor: This register defines the color frame buffer line length and pitch.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CFBLR)

For information about available fields see [`mod@ltdc_l2cfblr`]
module*/
pub type LTDC_L2CFBLR = crate::Reg<ltdc_l2cfblr::LTDC_L2CFBLRrs>;
///This register defines the color frame buffer line length and pitch.
pub mod ltdc_l2cfblr;
/**LTDC_L2CFBLNR (rw) register accessor: This register defines the number of lines in the color frame buffer.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CFBLNR)

For information about available fields see [`mod@ltdc_l2cfblnr`]
module*/
pub type LTDC_L2CFBLNR = crate::Reg<ltdc_l2cfblnr::LTDC_L2CFBLNRrs>;
///This register defines the number of lines in the color frame buffer.
pub mod ltdc_l2cfblnr;
/**LTDC_L2CLUTWR (w) register accessor: This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2CLUTWR)

For information about available fields see [`mod@ltdc_l2clutwr`]
module*/
pub type LTDC_L2CLUTWR = crate::Reg<ltdc_l2clutwr::LTDC_L2CLUTWRrs>;
///This register defines the CLUT address and the RGB value.
pub mod ltdc_l2clutwr;
