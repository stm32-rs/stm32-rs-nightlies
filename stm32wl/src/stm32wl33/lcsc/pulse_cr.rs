///Register `PULSE_CR` reader
pub type R = crate::R<PULSE_CRrs>;
///Register `PULSE_CR` writer
pub type W = crate::W<PULSE_CRrs>;
///Field `LCAB_PULSE_WIDTH` reader - Low Pulse Width for LCA and LCB
pub type LCAB_PULSE_WIDTH_R = crate::FieldReader;
///Field `LCAB_PULSE_WIDTH` writer - Low Pulse Width for LCA and LCB
pub type LCAB_PULSE_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LCT_PULSE_WIDTH` reader - Low Pulse Width for LCT
pub type LCT_PULSE_WIDTH_R = crate::FieldReader;
///Field `LCT_PULSE_WIDTH` writer - Low Pulse Width for LCT
pub type LCT_PULSE_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Low Pulse Width for LCA and LCB
    #[inline(always)]
    pub fn lcab_pulse_width(&self) -> LCAB_PULSE_WIDTH_R {
        LCAB_PULSE_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Low Pulse Width for LCT
    #[inline(always)]
    pub fn lct_pulse_width(&self) -> LCT_PULSE_WIDTH_R {
        LCT_PULSE_WIDTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PULSE_CR")
            .field("lcab_pulse_width", &self.lcab_pulse_width())
            .field("lct_pulse_width", &self.lct_pulse_width())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Low Pulse Width for LCA and LCB
    #[inline(always)]
    pub fn lcab_pulse_width(&mut self) -> LCAB_PULSE_WIDTH_W<'_, PULSE_CRrs> {
        LCAB_PULSE_WIDTH_W::new(self, 0)
    }
    ///Bits 8:11 - Low Pulse Width for LCT
    #[inline(always)]
    pub fn lct_pulse_width(&mut self) -> LCT_PULSE_WIDTH_W<'_, PULSE_CRrs> {
        LCT_PULSE_WIDTH_W::new(self, 8)
    }
}
/**LCSC_PULSE_CR register

You can [`read`](crate::Reg::read) this register and get [`pulse_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:PULSE_CR)*/
pub struct PULSE_CRrs;
impl crate::RegisterSpec for PULSE_CRrs {
    type Ux = u32;
}
///`read()` method returns [`pulse_cr::R`](R) reader structure
impl crate::Readable for PULSE_CRrs {}
///`write(|w| ..)` method takes [`pulse_cr::W`](W) writer structure
impl crate::Writable for PULSE_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PULSE_CR to value 0x70
impl crate::Resettable for PULSE_CRrs {
    const RESET_VALUE: u32 = 0x70;
}
