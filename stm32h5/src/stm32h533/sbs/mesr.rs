///Register `MESR` reader
pub type R = crate::R<MESRrs>;
///Register `MESR` writer
pub type W = crate::W<MESRrs>;
///Field `MCLR` reader - device memories erase status
pub type MCLR_R = crate::BitReader;
///Field `MCLR` writer - device memories erase status
pub type MCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPMEE` reader - ICACHE erase status
pub type IPMEE_R = crate::BitReader;
///Field `IPMEE` writer - ICACHE erase status
pub type IPMEE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - device memories erase status
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - ICACHE erase status
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESR")
            .field("mclr", &self.mclr())
            .field("ipmee", &self.ipmee())
            .finish()
    }
}
impl W {
    ///Bit 0 - device memories erase status
    #[inline(always)]
    pub fn mclr(&mut self) -> MCLR_W<'_, MESRrs> {
        MCLR_W::new(self, 0)
    }
    ///Bit 16 - ICACHE erase status
    #[inline(always)]
    pub fn ipmee(&mut self) -> IPMEE_W<'_, MESRrs> {
        IPMEE_W::new(self, 16)
    }
}
/**SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#SBS:MESR)*/
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
///`read()` method returns [`mesr::R`](R) reader structure
impl crate::Readable for MESRrs {}
///`write(|w| ..)` method takes [`mesr::W`](W) writer structure
impl crate::Writable for MESRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MESRrs {}
