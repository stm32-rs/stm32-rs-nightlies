///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
///Field `RAMCFGEN` reader - RAMCFG enable
pub type RAMCFGEN_R = crate::BitReader;
///Field `RAMCFGEN` writer - RAMCFG enable
pub type RAMCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1EN` reader - MDF1 enable
pub type MDF1EN_R = crate::BitReader;
///Field `MDF1EN` writer - MDF1 enable
pub type MDF1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1EN` reader - ADF enable
pub type ADF1EN_R = crate::BitReader;
///Field `ADF1EN` writer - ADF enable
pub type ADF1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - RAMCFG enable
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - MDF1 enable
    #[inline(always)]
    pub fn mdf1en(&self) -> MDF1EN_R {
        MDF1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADF enable
    #[inline(always)]
    pub fn adf1en(&self) -> ADF1EN_R {
        ADF1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("ramcfgen", &self.ramcfgen())
            .field("mdf1en", &self.mdf1en())
            .field("adf1en", &self.adf1en())
            .finish()
    }
}
impl W {
    ///Bit 12 - RAMCFG enable
    #[inline(always)]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<'_, AHB2ENRrs> {
        RAMCFGEN_W::new(self, 12)
    }
    ///Bit 16 - MDF1 enable
    #[inline(always)]
    pub fn mdf1en(&mut self) -> MDF1EN_W<'_, AHB2ENRrs> {
        MDF1EN_W::new(self, 16)
    }
    ///Bit 17 - ADF enable
    #[inline(always)]
    pub fn adf1en(&mut self) -> ADF1EN_W<'_, AHB2ENRrs> {
        ADF1EN_W::new(self, 17)
    }
}
/**RCC AHB2 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0x1000
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0x1000;
}
