///Register `EXTSRR` reader
pub type R = crate::R<EXTSRRrs>;
///Register `EXTSRR` writer
pub type W = crate::W<EXTSRRrs>;
///Field `DEEPSTOPF` reader - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field
pub type DEEPSTOPF_R = crate::BitReader;
///Field `DEEPSTOPF` writer - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field
pub type DEEPSTOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFPHASEF` reader - RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP.
pub type RFPHASEF_R = crate::BitReader;
///Field `RFPHASEF` writer - RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP.
pub type RFPHASEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 9 - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field
    #[inline(always)]
    pub fn deepstopf(&self) -> DEEPSTOPF_R {
        DEEPSTOPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP.
    #[inline(always)]
    pub fn rfphasef(&self) -> RFPHASEF_R {
        RFPHASEF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTSRR")
            .field("deepstopf", &self.deepstopf())
            .field("rfphasef", &self.rfphasef())
            .finish()
    }
}
impl W {
    ///Bit 9 - DEEPSTOPF System DeepStop Flag This bit is set by hardware and cleared only by a POR reset or by writing '1' in this bit field
    #[inline(always)]
    pub fn deepstopf(&mut self) -> DEEPSTOPF_W<'_, EXTSRRrs> {
        DEEPSTOPF_W::new(self, 9)
    }
    ///Bit 10 - RFPHASEF RFPHASE Flag This bit is set by hardware after a Radio wake-up event (BLE activation); it is cleared either by software, writing '1' in this bit field, or by hardware when Ready2Sleep signal is asserted by the Radio IP.
    #[inline(always)]
    pub fn rfphasef(&mut self) -> RFPHASEF_W<'_, EXTSRRrs> {
        RFPHASEF_W::new(self, 10)
    }
}
/**EXTSRR register

You can [`read`](crate::Reg::read) this register and get [`extsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:EXTSRR)*/
pub struct EXTSRRrs;
impl crate::RegisterSpec for EXTSRRrs {
    type Ux = u32;
}
///`read()` method returns [`extsrr::R`](R) reader structure
impl crate::Readable for EXTSRRrs {}
///`write(|w| ..)` method takes [`extsrr::W`](W) writer structure
impl crate::Writable for EXTSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTSRR to value 0
impl crate::Resettable for EXTSRRrs {}
