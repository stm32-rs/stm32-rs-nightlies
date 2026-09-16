///Register `NEXTLR` reader
pub type R = crate::R<NEXTLRrs>;
///Register `NEXTLR` writer
pub type W = crate::W<NEXTLRrs>;
///Field `INCR` reader - Increment
pub type INCR_R = crate::FieldReader;
///Field `INCR` writer - Increment
pub type INCR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Increment
    #[inline(always)]
    pub fn incr(&self) -> INCR_R {
        INCR_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NEXTLR")
            .field("incr", &self.incr())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Increment
    #[inline(always)]
    pub fn incr(&mut self) -> INCR_W<'_, NEXTLRrs> {
        INCR_W::new(self, 0)
    }
}
/**BSEC Next HDPL

You can [`read`](crate::Reg::read) this register and get [`nextlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:NEXTLR)*/
pub struct NEXTLRrs;
impl crate::RegisterSpec for NEXTLRrs {
    type Ux = u32;
}
///`read()` method returns [`nextlr::R`](R) reader structure
impl crate::Readable for NEXTLRrs {}
///`write(|w| ..)` method takes [`nextlr::W`](W) writer structure
impl crate::Writable for NEXTLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NEXTLR to value 0
impl crate::Resettable for NEXTLRrs {}
