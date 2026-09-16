///Register `HAINTMSK` reader
pub type R = crate::R<HAINTMSKrs>;
///Register `HAINTMSK` writer
pub type W = crate::W<HAINTMSKrs>;
///Field `HAINTM` reader - Channel interrupt mask
pub type HAINTM_R = crate::FieldReader<u16>;
///Field `HAINTM` writer - Channel interrupt mask
pub type HAINTM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINTMSK")
            .field("haintm", &self.haintm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Channel interrupt mask
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W<'_, HAINTMSKrs> {
        HAINTM_W::new(self, 0)
    }
}
/**OTG_FS host all channels interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`haintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#OTG_FS_HOST:HAINTMSK)*/
pub struct HAINTMSKrs;
impl crate::RegisterSpec for HAINTMSKrs {
    type Ux = u32;
}
///`read()` method returns [`haintmsk::R`](R) reader structure
impl crate::Readable for HAINTMSKrs {}
///`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure
impl crate::Writable for HAINTMSKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HAINTMSK to value 0
impl crate::Resettable for HAINTMSKrs {}
