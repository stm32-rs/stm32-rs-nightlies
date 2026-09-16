///Register `FCCRR` reader
pub type R = crate::R<FCCRRrs>;
///Register `FCCRR` writer
pub type W = crate::W<FCCRRrs>;
///Field `RELOAD` reader - reload value Reload value of the frame clock counter.
pub type RELOAD_R = crate::FieldReader<u16>;
///Field `RELOAD` writer - reload value Reload value of the frame clock counter.
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - reload value Reload value of the frame clock counter.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCRR")
            .field("reload", &self.reload())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - reload value Reload value of the frame clock counter.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, FCCRRrs> {
        RELOAD_W::new(self, 0)
    }
}
/**GFXTIM frame clock counter reload register

You can [`read`](crate::Reg::read) this register and get [`fccrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:FCCRR)*/
pub struct FCCRRrs;
impl crate::RegisterSpec for FCCRRrs {
    type Ux = u32;
}
///`read()` method returns [`fccrr::R`](R) reader structure
impl crate::Readable for FCCRRrs {}
///`write(|w| ..)` method takes [`fccrr::W`](W) writer structure
impl crate::Writable for FCCRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCCRR to value 0
impl crate::Resettable for FCCRRrs {}
