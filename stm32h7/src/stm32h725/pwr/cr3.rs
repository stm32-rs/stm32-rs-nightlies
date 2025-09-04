///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `BYPASS` reader - Power management unit bypass
pub type BYPASS_R = crate::BitReader;
///Field `BYPASS` writer - Power management unit bypass
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDOEN` reader - Low drop-out regulator enable
pub type LDOEN_R = crate::BitReader;
///Field `LDOEN` writer - Low drop-out regulator enable
pub type LDOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - SD converter Enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - SD converter Enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEXTHP` reader - SMPS step-down converter forced ON and in High Power MR mode
pub type SDEXTHP_R = crate::BitReader;
///Field `SDEXTHP` writer - SMPS step-down converter forced ON and in High Power MR mode
pub type SDEXTHP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDLEVEL` reader - SMPS step-down converter voltage output level selection
pub type SDLEVEL_R = crate::FieldReader;
///Field `SDLEVEL` writer - SMPS step-down converter voltage output level selection
pub type SDLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VBE` reader - VBAT charging enable
pub type VBE_R = crate::BitReader;
///Field `VBE` writer - VBAT charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBRS` reader - VBAT charging resistor selection
pub type VBRS_R = crate::BitReader;
///Field `VBRS` writer - VBAT charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEXTRDY` reader - SMPS step-down converter external supply ready
pub type SDEXTRDY_R = crate::BitReader;
///Field `SDEXTRDY` writer - SMPS step-down converter external supply ready
pub type SDEXTRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33DEN` writer - VDD33USB voltage level detector enable.
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBREGEN` reader - USB regulator enable.
pub type USBREGEN_R = crate::BitReader;
///Field `USBREGEN` writer - USB regulator enable.
pub type USBREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33RDY` reader - USB supply ready.
pub type USB33RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Power management unit bypass
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low drop-out regulator enable
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SD converter Enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SMPS step-down converter forced ON and in High Power MR mode
    #[inline(always)]
    pub fn sdexthp(&self) -> SDEXTHP_R {
        SDEXTHP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - SMPS step-down converter voltage output level selection
    #[inline(always)]
    pub fn sdlevel(&self) -> SDLEVEL_R {
        SDLEVEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - SMPS step-down converter external supply ready
    #[inline(always)]
    pub fn sdextrdy(&self) -> SDEXTRDY_R {
        SDEXTRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB supply ready.
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("bypass", &self.bypass())
            .field("ldoen", &self.ldoen())
            .field("sden", &self.sden())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .field("usbregen", &self.usbregen())
            .field("usb33rdy", &self.usb33rdy())
            .field("sdexthp", &self.sdexthp())
            .field("sdlevel", &self.sdlevel())
            .field("sdextrdy", &self.sdextrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power management unit bypass
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<CR3rs> {
        BYPASS_W::new(self, 0)
    }
    ///Bit 1 - Low drop-out regulator enable
    #[inline(always)]
    pub fn ldoen(&mut self) -> LDOEN_W<CR3rs> {
        LDOEN_W::new(self, 1)
    }
    ///Bit 2 - SD converter Enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<CR3rs> {
        SDEN_W::new(self, 2)
    }
    ///Bit 3 - SMPS step-down converter forced ON and in High Power MR mode
    #[inline(always)]
    pub fn sdexthp(&mut self) -> SDEXTHP_W<CR3rs> {
        SDEXTHP_W::new(self, 3)
    }
    ///Bits 4:5 - SMPS step-down converter voltage output level selection
    #[inline(always)]
    pub fn sdlevel(&mut self) -> SDLEVEL_W<CR3rs> {
        SDLEVEL_W::new(self, 4)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<CR3rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<CR3rs> {
        VBRS_W::new(self, 9)
    }
    ///Bit 16 - SMPS step-down converter external supply ready
    #[inline(always)]
    pub fn sdextrdy(&mut self) -> SDEXTRDY_W<CR3rs> {
        SDEXTRDY_W::new(self, 16)
    }
    ///Bit 24 - VDD33USB voltage level detector enable.
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W<CR3rs> {
        USB33DEN_W::new(self, 24)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    pub fn usbregen(&mut self) -> USBREGEN_W<CR3rs> {
        USBREGEN_W::new(self, 25)
    }
}
/**Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x06
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x06;
}
