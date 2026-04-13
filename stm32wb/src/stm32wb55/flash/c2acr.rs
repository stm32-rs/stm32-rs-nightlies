///Register `C2ACR` reader
pub type R = crate::R<C2ACRrs>;
///Register `C2ACR` writer
pub type W = crate::W<C2ACRrs>;
///Field `PRFTEN` reader - CPU2 cortex M0 prefetch enable
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - CPU2 cortex M0 prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICEN` reader - CPU2 cortex M0 instruction cache enable
pub type ICEN_R = crate::BitReader;
///Field `ICEN` writer - CPU2 cortex M0 instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRST` reader - CPU2 cortex M0 instruction cache reset
pub type ICRST_R = crate::BitReader;
///Field `ICRST` writer - CPU2 cortex M0 instruction cache reset
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PES` reader - CPU2 cortex M0 program erase suspend request
pub type PES_R = crate::BitReader;
///Field `PES` writer - CPU2 cortex M0 program erase suspend request
pub type PES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - CPU2 cortex M0 prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU2 cortex M0 instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU2 cortex M0 instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - CPU2 cortex M0 program erase suspend request
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2ACR")
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("icrst", &self.icrst())
            .field("pes", &self.pes())
            .finish()
    }
}
impl W {
    ///Bit 8 - CPU2 cortex M0 prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, C2ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - CPU2 cortex M0 instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<'_, C2ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 11 - CPU2 cortex M0 instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<'_, C2ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 15 - CPU2 cortex M0 program erase suspend request
    #[inline(always)]
    pub fn pes(&mut self) -> PES_W<'_, C2ACRrs> {
        PES_W::new(self, 15)
    }
}
/**CPU2 cortex M0 access control register

You can [`read`](crate::Reg::read) this register and get [`c2acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:C2ACR)*/
pub struct C2ACRrs;
impl crate::RegisterSpec for C2ACRrs {
    type Ux = u32;
}
///`read()` method returns [`c2acr::R`](R) reader structure
impl crate::Readable for C2ACRrs {}
///`write(|w| ..)` method takes [`c2acr::W`](W) writer structure
impl crate::Writable for C2ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2ACR to value 0x0600
impl crate::Resettable for C2ACRrs {
    const RESET_VALUE: u32 = 0x0600;
}
