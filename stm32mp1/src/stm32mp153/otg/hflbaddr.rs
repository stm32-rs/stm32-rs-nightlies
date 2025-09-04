///Register `HFLBADDR` reader
pub type R = crate::R<HFLBADDRrs>;
///Register `HFLBADDR` writer
pub type W = crate::W<HFLBADDRrs>;
///Field `HFLBADDR` reader - HFLBADDR
pub type HFLBADDR_R = crate::FieldReader<u32>;
///Field `HFLBADDR` writer - HFLBADDR
pub type HFLBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HFLBADDR
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFLBADDR")
            .field("hflbaddr", &self.hflbaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HFLBADDR
    #[inline(always)]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<HFLBADDRrs> {
        HFLBADDR_W::new(self, 0)
    }
}
/**This register holds the starting address of the frame list information (scatter/gather mode).

You can [`read`](crate::Reg::read) this register and get [`hflbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HFLBADDR)*/
pub struct HFLBADDRrs;
impl crate::RegisterSpec for HFLBADDRrs {
    type Ux = u32;
}
///`read()` method returns [`hflbaddr::R`](R) reader structure
impl crate::Readable for HFLBADDRrs {}
///`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure
impl crate::Writable for HFLBADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HFLBADDR to value 0
impl crate::Resettable for HFLBADDRrs {}
