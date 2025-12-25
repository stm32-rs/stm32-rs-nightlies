///Register `GETCAPR` reader
pub type R = crate::R<GETCAPRrs>;
///Register `GETCAPR` writer
pub type W = crate::W<GETCAPRrs>;
///Field `CAPPEND` reader - IBI MDB support for pending read notification
pub type CAPPEND_R = crate::BitReader;
///Field `CAPPEND` writer - IBI MDB support for pending read notification
pub type CAPPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - IBI MDB support for pending read notification
    #[inline(always)]
    pub fn cappend(&self) -> CAPPEND_R {
        CAPPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GETCAPR")
            .field("cappend", &self.cappend())
            .finish()
    }
}
impl W {
    ///Bit 14 - IBI MDB support for pending read notification
    #[inline(always)]
    pub fn cappend(&mut self) -> CAPPEND_W<'_, GETCAPRrs> {
        CAPPEND_W::new(self, 14)
    }
}
/**I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`getcapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getcapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#I3C1:GETCAPR)*/
pub struct GETCAPRrs;
impl crate::RegisterSpec for GETCAPRrs {
    type Ux = u32;
}
///`read()` method returns [`getcapr::R`](R) reader structure
impl crate::Readable for GETCAPRrs {}
///`write(|w| ..)` method takes [`getcapr::W`](W) writer structure
impl crate::Writable for GETCAPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GETCAPR to value 0
impl crate::Resettable for GETCAPRrs {}
