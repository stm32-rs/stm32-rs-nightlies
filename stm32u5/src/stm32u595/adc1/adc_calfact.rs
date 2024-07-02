///Register `ADC_CALFACT` reader
pub type R = crate::R<ADC_CALFACTrs>;
///Register `ADC_CALFACT` writer
pub type W = crate::W<ADC_CALFACTrs>;
///Field `I_APB_ADDR` reader - I_APB_ADDR
pub type I_APB_ADDR_R = crate::FieldReader;
///Field `I_APB_DATA` reader - I_APB_DATA
pub type I_APB_DATA_R = crate::FieldReader;
///Field `VALIDITY` reader - VALIDITY
pub type VALIDITY_R = crate::BitReader;
///Field `LATCH_COEF` reader - LATCH_COEF
pub type LATCH_COEF_R = crate::BitReader;
///Field `LATCH_COEF` writer - LATCH_COEF
pub type LATCH_COEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAPTURE_COEF` reader - CAPTURE_COEF
pub type CAPTURE_COEF_R = crate::BitReader;
///Field `CAPTURE_COEF` writer - CAPTURE_COEF
pub type CAPTURE_COEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - I_APB_ADDR
    #[inline(always)]
    pub fn i_apb_addr(&self) -> I_APB_ADDR_R {
        I_APB_ADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - I_APB_DATA
    #[inline(always)]
    pub fn i_apb_data(&self) -> I_APB_DATA_R {
        I_APB_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - VALIDITY
    #[inline(always)]
    pub fn validity(&self) -> VALIDITY_R {
        VALIDITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - LATCH_COEF
    #[inline(always)]
    pub fn latch_coef(&self) -> LATCH_COEF_R {
        LATCH_COEF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAPTURE_COEF
    #[inline(always)]
    pub fn capture_coef(&self) -> CAPTURE_COEF_R {
        CAPTURE_COEF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALFACT")
            .field("capture_coef", &self.capture_coef())
            .field("latch_coef", &self.latch_coef())
            .field("validity", &self.validity())
            .field("i_apb_data", &self.i_apb_data())
            .field("i_apb_addr", &self.i_apb_addr())
            .finish()
    }
}
impl W {
    ///Bit 24 - LATCH_COEF
    #[inline(always)]
    #[must_use]
    pub fn latch_coef(&mut self) -> LATCH_COEF_W<ADC_CALFACTrs> {
        LATCH_COEF_W::new(self, 24)
    }
    ///Bit 25 - CAPTURE_COEF
    #[inline(always)]
    #[must_use]
    pub fn capture_coef(&mut self) -> CAPTURE_COEF_W<ADC_CALFACTrs> {
        CAPTURE_COEF_W::new(self, 25)
    }
}
/**ADC user control register

You can [`read`](crate::Reg::read) this register and get [`adc_calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#ADC1:ADC_CALFACT)*/
pub struct ADC_CALFACTrs;
impl crate::RegisterSpec for ADC_CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`adc_calfact::R`](R) reader structure
impl crate::Readable for ADC_CALFACTrs {}
///`write(|w| ..)` method takes [`adc_calfact::W`](W) writer structure
impl crate::Writable for ADC_CALFACTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CALFACT to value 0
impl crate::Resettable for ADC_CALFACTrs {
    const RESET_VALUE: u32 = 0;
}
