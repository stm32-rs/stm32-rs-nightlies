#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    idr: IDR,
    lcr: LCR,
    sscr: SSCR,
    bpcr: BPCR,
    awcr: AWCR,
    twcr: TWCR,
    gcr: GCR,
    gc1r: GC1R,
    gc2r: GC2R,
    srcr: SRCR,
    _reserved10: [u8; 0x04],
    bccr: BCCR,
    _reserved11: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    lipcr: LIPCR,
    cpsr: CPSR,
    cdsr: CDSR,
    _reserved17: [u8; 0x38],
    l1cr: L1CR,
    l1whpcr: L1WHPCR,
    l1wvpcr: L1WVPCR,
    l1ckcr: L1CKCR,
    l1pfcr: L1PFCR,
    l1cacr: L1CACR,
    l1dccr: L1DCCR,
    l1bfcr: L1BFCR,
    _reserved25: [u8; 0x08],
    l1cfbar: L1CFBAR,
    l1cfblr: L1CFBLR,
    l1cfblnr: L1CFBLNR,
    _reserved28: [u8; 0x0c],
    l1clutwr: L1CLUTWR,
    _reserved29: [u8; 0x3c],
    l2cr: L2CR,
    l2whpcr: L2WHPCR,
    l2wvpcr: L2WVPCR,
    l2ckcr: L2CKCR,
    l2pfcr: L2PFCR,
    l2cacr: L2CACR,
    l2dccr: L2DCCR,
    l2bfcr: L2BFCR,
    _reserved37: [u8; 0x08],
    l2cfbar: L2CFBAR,
    l2cfblr: L2CFBLR,
    l2cfblnr: L2CFBLNR,
    _reserved40: [u8; 0x0c],
    l2clutwr: L2CLUTWR,
}
impl RegisterBlock {
    ///0x00 - LTDC identification register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x04 - LDTC layer count register
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    ///0x08 - This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    ///0x0c - This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    ///0x10 - This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    ///0x14 - This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    ///0x18 - This register defines the global configuration of the LCD-TFT controller.
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x1c - LTDC global configuration 1 register
    #[inline(always)]
    pub const fn gc1r(&self) -> &GC1R {
        &self.gc1r
    }
    ///0x20 - LTDC global configuration 2 register
    #[inline(always)]
    pub const fn gc2r(&self) -> &GC2R {
        &self.gc2r
    }
    ///0x24 - This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    ///0x2c - This register defines the background color (RGB888).
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    ///0x34 - This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x38 - This register returns the interrupt status flag.
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x3c - LTDC Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x40 - This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    ///0x44 - LTDC current position status register
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    ///0x48 - This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    ///0x84 - LTDC layer 1 control register
    #[inline(always)]
    pub const fn l1cr(&self) -> &L1CR {
        &self.l1cr
    }
    ///0x88 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.
    #[inline(always)]
    pub const fn l1whpcr(&self) -> &L1WHPCR {
        &self.l1whpcr
    }
    ///0x8c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.
    #[inline(always)]
    pub const fn l1wvpcr(&self) -> &L1WVPCR {
        &self.l1wvpcr
    }
    ///0x90 - This register defines the color key value (RGB), that is used by the color keying.
    #[inline(always)]
    pub const fn l1ckcr(&self) -> &L1CKCR {
        &self.l1ckcr
    }
    ///0x94 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    #[inline(always)]
    pub const fn l1pfcr(&self) -> &L1PFCR {
        &self.l1pfcr
    }
    ///0x98 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    #[inline(always)]
    pub const fn l1cacr(&self) -> &L1CACR {
        &self.l1cacr
    }
    ///0x9c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    #[inline(always)]
    pub const fn l1dccr(&self) -> &L1DCCR {
        &self.l1dccr
    }
    ///0xa0 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    #[inline(always)]
    pub const fn l1bfcr(&self) -> &L1BFCR {
        &self.l1bfcr
    }
    ///0xac - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    #[inline(always)]
    pub const fn l1cfbar(&self) -> &L1CFBAR {
        &self.l1cfbar
    }
    ///0xb0 - This register defines the color frame buffer line length and pitch.
    #[inline(always)]
    pub const fn l1cfblr(&self) -> &L1CFBLR {
        &self.l1cfblr
    }
    ///0xb4 - This register defines the number of lines in the color frame buffer.
    #[inline(always)]
    pub const fn l1cfblnr(&self) -> &L1CFBLNR {
        &self.l1cfblnr
    }
    ///0xc4 - This register defines the CLUT address and the RGB value.
    #[inline(always)]
    pub const fn l1clutwr(&self) -> &L1CLUTWR {
        &self.l1clutwr
    }
    ///0x104 - LTDC layer 2 control register
    #[inline(always)]
    pub const fn l2cr(&self) -> &L2CR {
        &self.l2cr
    }
    ///0x108 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.
    #[inline(always)]
    pub const fn l2whpcr(&self) -> &L2WHPCR {
        &self.l2whpcr
    }
    ///0x10c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.
    #[inline(always)]
    pub const fn l2wvpcr(&self) -> &L2WVPCR {
        &self.l2wvpcr
    }
    ///0x110 - This register defines the color key value (RGB), that is used by the color keying.
    #[inline(always)]
    pub const fn l2ckcr(&self) -> &L2CKCR {
        &self.l2ckcr
    }
    ///0x114 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
    #[inline(always)]
    pub const fn l2pfcr(&self) -> &L2PFCR {
        &self.l2pfcr
    }
    ///0x118 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
    #[inline(always)]
    pub const fn l2cacr(&self) -> &L2CACR {
        &self.l2cacr
    }
    ///0x11c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
    #[inline(always)]
    pub const fn l2dccr(&self) -> &L2DCCR {
        &self.l2dccr
    }
    ///0x120 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
    #[inline(always)]
    pub const fn l2bfcr(&self) -> &L2BFCR {
        &self.l2bfcr
    }
    ///0x12c - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
    #[inline(always)]
    pub const fn l2cfbar(&self) -> &L2CFBAR {
        &self.l2cfbar
    }
    ///0x130 - This register defines the color frame buffer line length and pitch.
    #[inline(always)]
    pub const fn l2cfblr(&self) -> &L2CFBLR {
        &self.l2cfblr
    }
    ///0x134 - This register defines the number of lines in the color frame buffer.
    #[inline(always)]
    pub const fn l2cfblnr(&self) -> &L2CFBLNR {
        &self.l2cfblnr
    }
    ///0x144 - This register defines the CLUT address and the RGB value.
    #[inline(always)]
    pub const fn l2clutwr(&self) -> &L2CLUTWR {
        &self.l2clutwr
    }
}
/**IDR (r) register accessor: LTDC identification register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///LTDC identification register
pub mod idr;
/**LCR (r) register accessor: LDTC layer count register

You can [`read`](crate::Reg::read) this register and get [`lcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LCR)

For information about available fields see [`mod@lcr`] module*/
pub type LCR = crate::Reg<lcr::LCRrs>;
///LDTC layer count register
pub mod lcr;
/**SSCR (rw) register accessor: This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:SSCR)

For information about available fields see [`mod@sscr`] module*/
pub type SSCR = crate::Reg<sscr::SSCRrs>;
///This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod sscr;
/**BPCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:BPCR)

For information about available fields see [`mod@bpcr`] module*/
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
///This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod bpcr;
/**AWCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:AWCR)

For information about available fields see [`mod@awcr`] module*/
pub type AWCR = crate::Reg<awcr::AWCRrs>;
///This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod awcr;
/**TWCR (rw) register accessor: This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.

You can [`read`](crate::Reg::read) this register and get [`twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:TWCR)

For information about available fields see [`mod@twcr`] module*/
pub type TWCR = crate::Reg<twcr::TWCRrs>;
///This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.
pub mod twcr;
/**GCR (rw) register accessor: This register defines the global configuration of the LCD-TFT controller.

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///This register defines the global configuration of the LCD-TFT controller.
pub mod gcr;
/**GC1R (r) register accessor: LTDC global configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`gc1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:GC1R)

For information about available fields see [`mod@gc1r`] module*/
pub type GC1R = crate::Reg<gc1r::GC1Rrs>;
///LTDC global configuration 1 register
pub mod gc1r;
/**GC2R (r) register accessor: LTDC global configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`gc2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:GC2R)

For information about available fields see [`mod@gc2r`] module*/
pub type GC2R = crate::Reg<gc2r::GC2Rrs>;
///LTDC global configuration 2 register
pub mod gc2r;
/**SRCR (rw) register accessor: This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.

You can [`read`](crate::Reg::read) this register and get [`srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:SRCR)

For information about available fields see [`mod@srcr`] module*/
pub type SRCR = crate::Reg<srcr::SRCRrs>;
///This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.
pub mod srcr;
/**BCCR (rw) register accessor: This register defines the background color (RGB888).

You can [`read`](crate::Reg::read) this register and get [`bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:BCCR)

For information about available fields see [`mod@bccr`] module*/
pub type BCCR = crate::Reg<bccr::BCCRrs>;
///This register defines the background color (RGB888).
pub mod bccr;
/**IER (rw) register accessor: This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod ier;
/**ISR (r) register accessor: This register returns the interrupt status flag.

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///This register returns the interrupt status flag.
pub mod isr;
/**ICR (w) register accessor: LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LTDC Interrupt Clear Register
pub mod icr;
/**LIPCR (rw) register accessor: This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.

You can [`read`](crate::Reg::read) this register and get [`lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LIPCR)

For information about available fields see [`mod@lipcr`] module*/
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
///This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
pub mod lipcr;
/**CPSR (r) register accessor: LTDC current position status register

You can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:CPSR)

For information about available fields see [`mod@cpsr`] module*/
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
///LTDC current position status register
pub mod cpsr;
/**CDSR (r) register accessor: This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:CDSR)

For information about available fields see [`mod@cdsr`] module*/
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
///This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.
pub mod cdsr;
/**L1CR (rw) register accessor: LTDC layer 1 control register

You can [`read`](crate::Reg::read) this register and get [`l1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CR)

For information about available fields see [`mod@l1cr`] module*/
pub type L1CR = crate::Reg<l1cr::L1CRrs>;
///LTDC layer 1 control register
pub mod l1cr;
/**L1WHPCR (rw) register accessor: This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`l1whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1WHPCR)

For information about available fields see [`mod@l1whpcr`] module*/
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCRrs>;
///This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.
pub mod l1whpcr;
/**L1WVPCR (rw) register accessor: This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`l1wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1WVPCR)

For information about available fields see [`mod@l1wvpcr`] module*/
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCRrs>;
///This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.
pub mod l1wvpcr;
/**L1CKCR (rw) register accessor: This register defines the color key value (RGB), that is used by the color keying.

You can [`read`](crate::Reg::read) this register and get [`l1ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CKCR)

For information about available fields see [`mod@l1ckcr`] module*/
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCRrs>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod l1ckcr;
/**L1PFCR (rw) register accessor: This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).

You can [`read`](crate::Reg::read) this register and get [`l1pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1PFCR)

For information about available fields see [`mod@l1pfcr`] module*/
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCRrs>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod l1pfcr;
/**L1CACR (rw) register accessor: This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`l1cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CACR)

For information about available fields see [`mod@l1cacr`] module*/
pub type L1CACR = crate::Reg<l1cacr::L1CACRrs>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod l1cacr;
/**L1DCCR (rw) register accessor: This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.

You can [`read`](crate::Reg::read) this register and get [`l1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1DCCR)

For information about available fields see [`mod@l1dccr`] module*/
pub type L1DCCR = crate::Reg<l1dccr::L1DCCRrs>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod l1dccr;
/**L1BFCR (rw) register accessor: This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color

You can [`read`](crate::Reg::read) this register and get [`l1bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1BFCR)

For information about available fields see [`mod@l1bfcr`] module*/
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCRrs>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod l1bfcr;
/**L1CFBAR (rw) register accessor: This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.

You can [`read`](crate::Reg::read) this register and get [`l1cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CFBAR)

For information about available fields see [`mod@l1cfbar`] module*/
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBARrs>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod l1cfbar;
/**L1CFBLR (rw) register accessor: This register defines the color frame buffer line length and pitch.

You can [`read`](crate::Reg::read) this register and get [`l1cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CFBLR)

For information about available fields see [`mod@l1cfblr`] module*/
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLRrs>;
///This register defines the color frame buffer line length and pitch.
pub mod l1cfblr;
/**L1CFBLNR (rw) register accessor: This register defines the number of lines in the color frame buffer.

You can [`read`](crate::Reg::read) this register and get [`l1cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CFBLNR)

For information about available fields see [`mod@l1cfblnr`] module*/
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNRrs>;
///This register defines the number of lines in the color frame buffer.
pub mod l1cfblnr;
/**L1CLUTWR (w) register accessor: This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CLUTWR)

For information about available fields see [`mod@l1clutwr`] module*/
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWRrs>;
///This register defines the CLUT address and the RGB value.
pub mod l1clutwr;
/**L2CR (rw) register accessor: LTDC layer 2 control register

You can [`read`](crate::Reg::read) this register and get [`l2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CR)

For information about available fields see [`mod@l2cr`] module*/
pub type L2CR = crate::Reg<l2cr::L2CRrs>;
///LTDC layer 2 control register
pub mod l2cr;
/**L2WHPCR (rw) register accessor: This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`l2whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2WHPCR)

For information about available fields see [`mod@l2whpcr`] module*/
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCRrs>;
///This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\] bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\] bits in the LTDC_AWCR register.
pub mod l2whpcr;
/**L2WVPCR (rw) register accessor: This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2WVPCR)

For information about available fields see [`mod@l2wvpcr`] module*/
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCRrs>;
///This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.
pub mod l2wvpcr;
/**L2CKCR (rw) register accessor: This register defines the color key value (RGB), that is used by the color keying.

You can [`read`](crate::Reg::read) this register and get [`l2ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CKCR)

For information about available fields see [`mod@l2ckcr`] module*/
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCRrs>;
///This register defines the color key value (RGB), that is used by the color keying.
pub mod l2ckcr;
/**L2PFCR (rw) register accessor: This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).

You can [`read`](crate::Reg::read) this register and get [`l2pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2PFCR)

For information about available fields see [`mod@l2pfcr`] module*/
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCRrs>;
///This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).
pub mod l2pfcr;
/**L2CACR (rw) register accessor: This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`l2cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CACR)

For information about available fields see [`mod@l2cacr`] module*/
pub type L2CACR = crate::Reg<l2cacr::L2CACRrs>;
///This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.
pub mod l2cacr;
/**L2DCCR (rw) register accessor: This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.

You can [`read`](crate::Reg::read) this register and get [`l2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2DCCR)

For information about available fields see [`mod@l2dccr`] module*/
pub type L2DCCR = crate::Reg<l2dccr::L2DCCRrs>;
///This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.
pub mod l2dccr;
/**L2BFCR (rw) register accessor: This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color

You can [`read`](crate::Reg::read) this register and get [`l2bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2BFCR)

For information about available fields see [`mod@l2bfcr`] module*/
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCRrs>;
///This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color
pub mod l2bfcr;
/**L2CFBAR (rw) register accessor: This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.

You can [`read`](crate::Reg::read) this register and get [`l2cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CFBAR)

For information about available fields see [`mod@l2cfbar`] module*/
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBARrs>;
///This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.
pub mod l2cfbar;
/**L2CFBLR (rw) register accessor: This register defines the color frame buffer line length and pitch.

You can [`read`](crate::Reg::read) this register and get [`l2cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CFBLR)

For information about available fields see [`mod@l2cfblr`] module*/
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLRrs>;
///This register defines the color frame buffer line length and pitch.
pub mod l2cfblr;
/**L2CFBLNR (rw) register accessor: This register defines the number of lines in the color frame buffer.

You can [`read`](crate::Reg::read) this register and get [`l2cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CFBLNR)

For information about available fields see [`mod@l2cfblnr`] module*/
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNRrs>;
///This register defines the number of lines in the color frame buffer.
pub mod l2cfblnr;
/**L2CLUTWR (w) register accessor: This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CLUTWR)

For information about available fields see [`mod@l2clutwr`] module*/
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWRrs>;
///This register defines the CLUT address and the RGB value.
pub mod l2clutwr;
