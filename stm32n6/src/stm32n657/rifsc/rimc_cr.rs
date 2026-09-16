///Register `RIMC_CR` reader
pub type R = crate::R<RIMC_CRrs>;
///Register `RIMC_CR` writer
pub type W = crate::W<RIMC_CRrs>;
///Field `GLOCK` reader - global lock
pub type GLOCK_R = crate::BitReader;
///Field `GLOCK` writer - global lock
pub type GLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAPCID` reader - debug access port compartment ID
pub type DAPCID_R = crate::FieldReader;
///Field `DAPCID` writer - debug access port compartment ID
pub type DAPCID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - global lock
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - debug access port compartment ID
    #[inline(always)]
    pub fn dapcid(&self) -> DAPCID_R {
        DAPCID_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIMC_CR")
            .field("glock", &self.glock())
            .field("dapcid", &self.dapcid())
            .finish()
    }
}
impl W {
    ///Bit 0 - global lock
    #[inline(always)]
    pub fn glock(&mut self) -> GLOCK_W<'_, RIMC_CRrs> {
        GLOCK_W::new(self, 0)
    }
    ///Bits 8:10 - debug access port compartment ID
    #[inline(always)]
    pub fn dapcid(&mut self) -> DAPCID_W<'_, RIMC_CRrs> {
        DAPCID_W::new(self, 8)
    }
}
/**RIFSC RIMC master configuration register

You can [`read`](crate::Reg::read) this register and get [`rimc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rimc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RIFSC:RIMC_CR)*/
pub struct RIMC_CRrs;
impl crate::RegisterSpec for RIMC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`rimc_cr::R`](R) reader structure
impl crate::Readable for RIMC_CRrs {}
///`write(|w| ..)` method takes [`rimc_cr::W`](W) writer structure
impl crate::Writable for RIMC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RIMC_CR to value 0x0710
impl crate::Resettable for RIMC_CRrs {
    const RESET_VALUE: u32 = 0x0710;
}
