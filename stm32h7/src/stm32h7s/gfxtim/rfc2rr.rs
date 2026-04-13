///Register `RFC2RR` reader
pub type R = crate::R<RFC2RRrs>;
///Register `RFC2RR` writer
pub type W = crate::W<RFC2RRrs>;
///Field `FRAME` reader - frame reload value Reload value for the relative frame counter 2.
pub type FRAME_R = crate::FieldReader<u16>;
///Field `FRAME` writer - frame reload value Reload value for the relative frame counter 2.
pub type FRAME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - frame reload value Reload value for the relative frame counter 2.
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFC2RR")
            .field("frame", &self.frame())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - frame reload value Reload value for the relative frame counter 2.
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<'_, RFC2RRrs> {
        FRAME_W::new(self, 0)
    }
}
/**GFXTIM relative frame counter 2 reload register

You can [`read`](crate::Reg::read) this register and get [`rfc2rr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc2rr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:RFC2RR)*/
pub struct RFC2RRrs;
impl crate::RegisterSpec for RFC2RRrs {
    type Ux = u32;
}
///`read()` method returns [`rfc2rr::R`](R) reader structure
impl crate::Readable for RFC2RRrs {}
///`write(|w| ..)` method takes [`rfc2rr::W`](W) writer structure
impl crate::Writable for RFC2RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFC2RR to value 0
impl crate::Resettable for RFC2RRrs {}
