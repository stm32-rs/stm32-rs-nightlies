///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `ETF` reader - External trigger filter
pub type ETF_R = crate::FieldReader;
///Field `ETF` writer - External trigger filter
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ETPS` reader - External trigger prescaler
pub type ETPS_R = crate::FieldReader;
///Field `ETPS` writer - External trigger prescaler
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECE` reader - External clock enable
pub type ECE_R = crate::BitReader;
///Field `ECE` writer - External clock enable
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETP` reader - External trigger polarity
pub type ETP_R = crate::BitReader;
///Field `ETP` writer - External trigger polarity
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("etf", &self.etf())
            .field("etps", &self.etps())
            .field("ece", &self.ece())
            .field("etp", &self.etp())
            .finish()
    }
}
impl W {
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, SMCRrs> {
        ETF_W::new(self, 8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W<'_, SMCRrs> {
        ETPS_W::new(self, 12)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W<'_, SMCRrs> {
        ECE_W::new(self, 14)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W<'_, SMCRrs> {
        ETP_W::new(self, 15)
    }
}
/**TIM10 slave mode control register

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#TIM10:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u16;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCRrs {}
