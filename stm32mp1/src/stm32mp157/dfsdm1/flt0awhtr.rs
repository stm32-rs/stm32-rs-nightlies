///Register `FLT0AWHTR` reader
pub type R = crate::R<FLT0AWHTRrs>;
///Register `FLT0AWHTR` writer
pub type W = crate::W<FLT0AWHTRrs>;
///Field `BKAWH` reader - BKAWH
pub type BKAWH_R = crate::FieldReader;
///Field `BKAWH` writer - BKAWH
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWHT` reader - AWHT
pub type AWHT_R = crate::FieldReader<u32>;
///Field `AWHT` writer - AWHT
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - BKAWH
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - AWHT
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0AWHTR")
            .field("bkawh", &self.bkawh())
            .field("awht", &self.awht())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BKAWH
    #[inline(always)]
    pub fn bkawh(&mut self) -> BKAWH_W<'_, FLT0AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    ///Bits 8:31 - AWHT
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W<'_, FLT0AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
/**DFSDM filter 0 analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`flt0awhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0awhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0AWHTR)*/
pub struct FLT0AWHTRrs;
impl crate::RegisterSpec for FLT0AWHTRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0awhtr::R`](R) reader structure
impl crate::Readable for FLT0AWHTRrs {}
///`write(|w| ..)` method takes [`flt0awhtr::W`](W) writer structure
impl crate::Writable for FLT0AWHTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT0AWHTR to value 0
impl crate::Resettable for FLT0AWHTRrs {}
