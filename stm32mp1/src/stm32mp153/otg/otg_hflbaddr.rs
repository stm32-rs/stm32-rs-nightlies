///Register `OTG_HFLBADDR` reader
pub type R = crate::R<OTG_HFLBADDRrs>;
///Register `OTG_HFLBADDR` writer
pub type W = crate::W<OTG_HFLBADDRrs>;
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
        f.debug_struct("OTG_HFLBADDR")
            .field("hflbaddr", &self.hflbaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HFLBADDR
    #[inline(always)]
    #[must_use]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<OTG_HFLBADDRrs> {
        HFLBADDR_W::new(self, 0)
    }
}
/**This register holds the starting address of the frame list information (scatter/gather mode).

You can [`read`](crate::Reg::read) this register and get [`otg_hflbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hflbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_HFLBADDR)*/
pub struct OTG_HFLBADDRrs;
impl crate::RegisterSpec for OTG_HFLBADDRrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hflbaddr::R`](R) reader structure
impl crate::Readable for OTG_HFLBADDRrs {}
///`write(|w| ..)` method takes [`otg_hflbaddr::W`](W) writer structure
impl crate::Writable for OTG_HFLBADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_HFLBADDR to value 0
impl crate::Resettable for OTG_HFLBADDRrs {
    const RESET_VALUE: u32 = 0;
}
