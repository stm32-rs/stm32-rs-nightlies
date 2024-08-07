///Register `HDP_CTRL` reader
pub type R = crate::R<HDP_CTRLrs>;
///Register `HDP_CTRL` writer
pub type W = crate::W<HDP_CTRLrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP_CTRL").field("en", &self.en()).finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<HDP_CTRLrs> {
        EN_W::new(self, 0)
    }
}
/**HDP Control

You can [`read`](crate::Reg::read) this register and get [`hdp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_CTRL)*/
pub struct HDP_CTRLrs;
impl crate::RegisterSpec for HDP_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`hdp_ctrl::R`](R) reader structure
impl crate::Readable for HDP_CTRLrs {}
///`write(|w| ..)` method takes [`hdp_ctrl::W`](W) writer structure
impl crate::Writable for HDP_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HDP_CTRL to value 0
impl crate::Resettable for HDP_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
