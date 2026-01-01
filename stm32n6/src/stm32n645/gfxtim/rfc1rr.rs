///Register `RFC1RR` reader
pub type R = crate::R<RFC1RRrs>;
///Register `RFC1RR` writer
pub type W = crate::W<RFC1RRrs>;
///Field `FRAME` reader - frame reload value
pub type FRAME_R = crate::FieldReader<u16>;
///Field `FRAME` writer - frame reload value
pub type FRAME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - frame reload value
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFC1RR")
            .field("frame", &self.frame())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - frame reload value
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<'_, RFC1RRrs> {
        FRAME_W::new(self, 0)
    }
}
/**GFXTIM relative frame counter 1 reload register

You can [`read`](crate::Reg::read) this register and get [`rfc1rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc1rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:RFC1RR)*/
pub struct RFC1RRrs;
impl crate::RegisterSpec for RFC1RRrs {
    type Ux = u32;
}
///`read()` method returns [`rfc1rr::R`](R) reader structure
impl crate::Readable for RFC1RRrs {}
///`write(|w| ..)` method takes [`rfc1rr::W`](W) writer structure
impl crate::Writable for RFC1RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFC1RR to value 0
impl crate::Resettable for RFC1RRrs {}
