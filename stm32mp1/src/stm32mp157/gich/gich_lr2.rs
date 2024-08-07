///Register `GICH_LR2` reader
pub type R = crate::R<GICH_LR2rs>;
///Register `GICH_LR2` writer
pub type W = crate::W<GICH_LR2rs>;
///Field `VIRTUALID` reader - VIRTUALID
pub type VIRTUALID_R = crate::FieldReader<u16>;
///Field `VIRTUALID` writer - VIRTUALID
pub type VIRTUALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PHYSICALID` reader - PHYSICALID
pub type PHYSICALID_R = crate::FieldReader<u16>;
///Field `PHYSICALID` writer - PHYSICALID
pub type PHYSICALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader;
///Field `PRIORITY` writer - PRIORITY
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `STATE` reader - STATE
pub type STATE_R = crate::FieldReader;
///Field `STATE` writer - STATE
pub type STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GRP1` reader - GRP1
pub type GRP1_R = crate::BitReader;
///Field `GRP1` writer - GRP1
pub type GRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW` reader - HW
pub type HW_R = crate::BitReader;
///Field `HW` writer - HW
pub type HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - HW
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICH_LR2")
            .field("virtualid", &self.virtualid())
            .field("physicalid", &self.physicalid())
            .field("priority", &self.priority())
            .field("state", &self.state())
            .field("grp1", &self.grp1())
            .field("hw", &self.hw())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - VIRTUALID
    #[inline(always)]
    #[must_use]
    pub fn virtualid(&mut self) -> VIRTUALID_W<GICH_LR2rs> {
        VIRTUALID_W::new(self, 0)
    }
    ///Bits 10:19 - PHYSICALID
    #[inline(always)]
    #[must_use]
    pub fn physicalid(&mut self) -> PHYSICALID_W<GICH_LR2rs> {
        PHYSICALID_W::new(self, 10)
    }
    ///Bits 23:27 - PRIORITY
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICH_LR2rs> {
        PRIORITY_W::new(self, 23)
    }
    ///Bits 28:29 - STATE
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<GICH_LR2rs> {
        STATE_W::new(self, 28)
    }
    ///Bit 30 - GRP1
    #[inline(always)]
    #[must_use]
    pub fn grp1(&mut self) -> GRP1_W<GICH_LR2rs> {
        GRP1_W::new(self, 30)
    }
    ///Bit 31 - HW
    #[inline(always)]
    #[must_use]
    pub fn hw(&mut self) -> HW_W<GICH_LR2rs> {
        HW_W::new(self, 31)
    }
}
/**GICH list register 2

You can [`read`](crate::Reg::read) this register and get [`gich_lr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gich_lr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICH:GICH_LR2)*/
pub struct GICH_LR2rs;
impl crate::RegisterSpec for GICH_LR2rs {
    type Ux = u32;
}
///`read()` method returns [`gich_lr2::R`](R) reader structure
impl crate::Readable for GICH_LR2rs {}
///`write(|w| ..)` method takes [`gich_lr2::W`](W) writer structure
impl crate::Writable for GICH_LR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICH_LR2 to value 0
impl crate::Resettable for GICH_LR2rs {
    const RESET_VALUE: u32 = 0;
}
