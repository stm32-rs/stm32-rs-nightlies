///Register `SFR` reader
pub type R = crate::R<SFRrs>;
///Register `SFR` writer
pub type W = crate::W<SFRrs>;
///Field `SFSA` reader - Secure flash start address
pub type SFSA_R = crate::FieldReader;
///Field `SFSA` writer - Secure flash start address
pub type SFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FSD` reader - Flash security disable
pub type FSD_R = crate::BitReader;
///Field `FSD` writer - Flash security disable
pub type FSD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDS` reader - Disable Cortex M0 debug access
pub type DDS_R = crate::BitReader;
///Field `DDS` writer - Disable Cortex M0 debug access
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Secure flash start address
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Flash security disable
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Disable Cortex M0 debug access
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFR")
            .field("sfsa", &self.sfsa())
            .field("dds", &self.dds())
            .field("fsd", &self.fsd())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Secure flash start address
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W<'_, SFRrs> {
        SFSA_W::new(self, 0)
    }
    ///Bit 8 - Flash security disable
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W<'_, SFRrs> {
        FSD_W::new(self, 8)
    }
    ///Bit 12 - Disable Cortex M0 debug access
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<'_, SFRrs> {
        DDS_W::new(self, 12)
    }
}
/**Secure flash start address register

You can [`read`](crate::Reg::read) this register and get [`sfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash:SFR)*/
pub struct SFRrs;
impl crate::RegisterSpec for SFRrs {
    type Ux = u32;
}
///`read()` method returns [`sfr::R`](R) reader structure
impl crate::Readable for SFRrs {}
///`write(|w| ..)` method takes [`sfr::W`](W) writer structure
impl crate::Writable for SFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFR to value 0xffff_ee00
impl crate::Resettable for SFRrs {
    const RESET_VALUE: u32 = 0xffff_ee00;
}
