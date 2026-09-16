///Register `AFC1_CONFIG` reader
pub type R = crate::R<AFC1_CONFIGrs>;
///Register `AFC1_CONFIG` writer
pub type W = crate::W<AFC1_CONFIGrs>;
///Field `AFC_FAST_PERIOD` reader - Length of the AFC fast period (in number of samples unit)
pub type AFC_FAST_PERIOD_R = crate::FieldReader;
///Field `AFC_FAST_PERIOD` writer - Length of the AFC fast period (in number of samples unit)
pub type AFC_FAST_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Length of the AFC fast period (in number of samples unit)
    #[inline(always)]
    pub fn afc_fast_period(&self) -> AFC_FAST_PERIOD_R {
        AFC_FAST_PERIOD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFC1_CONFIG")
            .field("afc_fast_period", &self.afc_fast_period())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Length of the AFC fast period (in number of samples unit)
    #[inline(always)]
    pub fn afc_fast_period(&mut self) -> AFC_FAST_PERIOD_W<'_, AFC1_CONFIGrs> {
        AFC_FAST_PERIOD_W::new(self, 0)
    }
}
/**AFC1_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC1_CONFIG)*/
pub struct AFC1_CONFIGrs;
impl crate::RegisterSpec for AFC1_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`afc1_config::R`](R) reader structure
impl crate::Readable for AFC1_CONFIGrs {}
///`write(|w| ..)` method takes [`afc1_config::W`](W) writer structure
impl crate::Writable for AFC1_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFC1_CONFIG to value 0x18
impl crate::Resettable for AFC1_CONFIGrs {
    const RESET_VALUE: u32 = 0x18;
}
