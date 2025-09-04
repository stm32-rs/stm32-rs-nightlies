///Register `SCD1CR` reader
pub type R = crate::R<SCD1CRrs>;
///Register `SCD1CR` writer
pub type W = crate::W<SCD1CRrs>;
///Field `SCDEN` reader - SCDx enable
pub type SCDEN_R = crate::BitReader;
///Field `SCDEN` writer - SCDx enable
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKSCD` reader - Break signal assignment for short circuit detector
pub type BKSCD_R = crate::FieldReader;
///Field `BKSCD` writer - Break signal assignment for short circuit detector
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCDT` reader - SCDx threshold
pub type SCDT_R = crate::FieldReader;
///Field `SCDT` writer - SCDx threshold
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCDACTIVE` reader - SCDx active flag
pub type SCDACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - SCDx enable
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:7 - Break signal assignment for short circuit detector
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:19 - SCDx threshold
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bit 31 - SCDx active flag
    #[inline(always)]
    pub fn scdactive(&self) -> SCDACTIVE_R {
        SCDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCD1CR")
            .field("scden", &self.scden())
            .field("bkscd", &self.bkscd())
            .field("scdt", &self.scdt())
            .field("scdactive", &self.scdactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - SCDx enable
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W<SCD1CRrs> {
        SCDEN_W::new(self, 0)
    }
    ///Bits 4:7 - Break signal assignment for short circuit detector
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<SCD1CRrs> {
        BKSCD_W::new(self, 4)
    }
    ///Bits 12:19 - SCDx threshold
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W<SCD1CRrs> {
        SCDT_W::new(self, 12)
    }
}
/**MDF short circuit detector control register 1

You can [`read`](crate::Reg::read) this register and get [`scd1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:SCD1CR)*/
pub struct SCD1CRrs;
impl crate::RegisterSpec for SCD1CRrs {
    type Ux = u32;
}
///`read()` method returns [`scd1cr::R`](R) reader structure
impl crate::Readable for SCD1CRrs {}
///`write(|w| ..)` method takes [`scd1cr::W`](W) writer structure
impl crate::Writable for SCD1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCD1CR to value 0
impl crate::Resettable for SCD1CRrs {}
