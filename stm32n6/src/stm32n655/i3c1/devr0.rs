///Register `DEVR0` reader
pub type R = crate::R<DEVR0rs>;
///Register `DEVR0` writer
pub type W = crate::W<DEVR0rs>;
///Field `DAVAL` reader - Dynamic address is valid (when the I3C acts as target)
pub type DAVAL_R = crate::BitReader;
///Field `DAVAL` writer - Dynamic address is valid (when the I3C acts as target)
pub type DAVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - 7-bit dynamic address
pub type DA_R = crate::FieldReader;
///Field `DA` writer - 7-bit dynamic address
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `IBIEN` reader - IBI request enable (when the I3C acts as target)
pub type IBIEN_R = crate::BitReader;
///Field `IBIEN` writer - IBI request enable (when the I3C acts as target)
pub type IBIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CREN` reader - Controller-role request enable (when the I3C acts as target)
pub type CREN_R = crate::BitReader;
///Field `CREN` writer - Controller-role request enable (when the I3C acts as target)
pub type CREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HJEN` reader - Hot-join request enable (when the I3C acts as target)
pub type HJEN_R = crate::BitReader;
///Field `HJEN` writer - Hot-join request enable (when the I3C acts as target)
pub type HJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AS` reader - Activity state (when the I3C acts as target)
pub type AS_R = crate::FieldReader;
///Field `RSTACT` reader - Reset action/level on received reset pattern (when the I3C acts as target)
pub type RSTACT_R = crate::FieldReader;
///Field `RSTVAL` reader - Reset action is valid (when the I3C acts as target)
pub type RSTVAL_R = crate::BitReader;
impl R {
    ///Bit 0 - Dynamic address is valid (when the I3C acts as target)
    #[inline(always)]
    pub fn daval(&self) -> DAVAL_R {
        DAVAL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - 7-bit dynamic address
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 16 - IBI request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn ibien(&self) -> IBIEN_R {
        IBIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Controller-role request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn cren(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Hot-join request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn hjen(&self) -> HJEN_R {
        HJEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Activity state (when the I3C acts as target)
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Reset action/level on received reset pattern (when the I3C acts as target)
    #[inline(always)]
    pub fn rstact(&self) -> RSTACT_R {
        RSTACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - Reset action is valid (when the I3C acts as target)
    #[inline(always)]
    pub fn rstval(&self) -> RSTVAL_R {
        RSTVAL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVR0")
            .field("daval", &self.daval())
            .field("da", &self.da())
            .field("ibien", &self.ibien())
            .field("cren", &self.cren())
            .field("hjen", &self.hjen())
            .field("as_", &self.as_())
            .field("rstact", &self.rstact())
            .field("rstval", &self.rstval())
            .finish()
    }
}
impl W {
    ///Bit 0 - Dynamic address is valid (when the I3C acts as target)
    #[inline(always)]
    pub fn daval(&mut self) -> DAVAL_W<'_, DEVR0rs> {
        DAVAL_W::new(self, 0)
    }
    ///Bits 1:7 - 7-bit dynamic address
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DEVR0rs> {
        DA_W::new(self, 1)
    }
    ///Bit 16 - IBI request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn ibien(&mut self) -> IBIEN_W<'_, DEVR0rs> {
        IBIEN_W::new(self, 16)
    }
    ///Bit 17 - Controller-role request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn cren(&mut self) -> CREN_W<'_, DEVR0rs> {
        CREN_W::new(self, 17)
    }
    ///Bit 19 - Hot-join request enable (when the I3C acts as target)
    #[inline(always)]
    pub fn hjen(&mut self) -> HJEN_W<'_, DEVR0rs> {
        HJEN_W::new(self, 19)
    }
}
/**I3C own device characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I3C1:DEVR0)*/
pub struct DEVR0rs;
impl crate::RegisterSpec for DEVR0rs {
    type Ux = u32;
}
///`read()` method returns [`devr0::R`](R) reader structure
impl crate::Readable for DEVR0rs {}
///`write(|w| ..)` method takes [`devr0::W`](W) writer structure
impl crate::Writable for DEVR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVR0 to value 0
impl crate::Resettable for DEVR0rs {}
