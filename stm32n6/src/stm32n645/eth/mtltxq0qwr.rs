///Register `MTLTXQ0QWR` reader
pub type R = crate::R<MTLTXQ0QWRrs>;
///Register `MTLTXQ0QWR` writer
pub type W = crate::W<MTLTXQ0QWRrs>;
///Field `ISCQW` reader - Weights
pub type ISCQW_R = crate::FieldReader;
///Field `ISCQW` writer - Weights
pub type ISCQW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Weights
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ0QWR")
            .field("iscqw", &self.iscqw())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Weights
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W<'_, MTLTXQ0QWRrs> {
        ISCQW_W::new(self, 0)
    }
}
/**Tx queue 0 quantum weight register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0qwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0qwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLTXQ0QWR)*/
pub struct MTLTXQ0QWRrs;
impl crate::RegisterSpec for MTLTXQ0QWRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq0qwr::R`](R) reader structure
impl crate::Readable for MTLTXQ0QWRrs {}
///`write(|w| ..)` method takes [`mtltxq0qwr::W`](W) writer structure
impl crate::Writable for MTLTXQ0QWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ0QWR to value 0
impl crate::Resettable for MTLTXQ0QWRrs {}
