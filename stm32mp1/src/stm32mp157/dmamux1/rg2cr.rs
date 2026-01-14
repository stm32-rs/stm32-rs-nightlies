///Register `RG2CR` reader
pub type R = crate::R<RG2CRrs>;
///Register `RG2CR` writer
pub type W = crate::W<RG2CRrs>;
///Field `SIG_ID` reader - SIG_ID
pub type SIG_ID_R = crate::FieldReader;
///Field `SIG_ID` writer - SIG_ID
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `OIE` reader - OIE
pub type OIE_R = crate::BitReader;
///Field `OIE` writer - OIE
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GE` reader - GE
pub type GE_R = crate::BitReader;
///Field `GE` writer - GE
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPOL` reader - GPOL
pub type GPOL_R = crate::FieldReader;
///Field `GPOL` writer - GPOL
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GNBREQ` reader - GNBREQ
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - GNBREQ
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:2 - SIG_ID
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - OIE
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - GE
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - GPOL
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - GNBREQ
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RG2CR")
            .field("sig_id", &self.sig_id())
            .field("oie", &self.oie())
            .field("ge", &self.ge())
            .field("gpol", &self.gpol())
            .field("gnbreq", &self.gnbreq())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SIG_ID
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<'_, RG2CRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - OIE
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<'_, RG2CRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - GE
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<'_, RG2CRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - GPOL
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, RG2CRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - GNBREQ
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<'_, RG2CRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**DMAMUX request generator channel 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`rg2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMAMUX1:RG2CR)*/
pub struct RG2CRrs;
impl crate::RegisterSpec for RG2CRrs {
    type Ux = u32;
}
///`read()` method returns [`rg2cr::R`](R) reader structure
impl crate::Readable for RG2CRrs {}
///`write(|w| ..)` method takes [`rg2cr::W`](W) writer structure
impl crate::Writable for RG2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RG2CR to value 0
impl crate::Resettable for RG2CRrs {}
