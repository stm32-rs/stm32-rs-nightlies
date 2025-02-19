///Register `HWCFGR7` reader
pub type R = crate::R<HWCFGR7rs>;
///Field `AF_PRIO0` reader - AF_PRIO0
pub type AF_PRIO0_R = crate::FieldReader;
///Field `AF_PRIO1` reader - AF_PRIO1
pub type AF_PRIO1_R = crate::FieldReader;
///Field `AF_PRIO2` reader - AF_PRIO2
pub type AF_PRIO2_R = crate::FieldReader;
///Field `AF_PRIO3` reader - AF_PRIO3
pub type AF_PRIO3_R = crate::FieldReader;
///Field `AF_PRIO4` reader - AF_PRIO4
pub type AF_PRIO4_R = crate::FieldReader;
///Field `AF_PRIO5` reader - AF_PRIO5
pub type AF_PRIO5_R = crate::FieldReader;
///Field `AF_PRIO6` reader - AF_PRIO6
pub type AF_PRIO6_R = crate::FieldReader;
///Field `AF_PRIO7` reader - AF_PRIO7
pub type AF_PRIO7_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AF_PRIO0
    #[inline(always)]
    pub fn af_prio0(&self) -> AF_PRIO0_R {
        AF_PRIO0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AF_PRIO1
    #[inline(always)]
    pub fn af_prio1(&self) -> AF_PRIO1_R {
        AF_PRIO1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AF_PRIO2
    #[inline(always)]
    pub fn af_prio2(&self) -> AF_PRIO2_R {
        AF_PRIO2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AF_PRIO3
    #[inline(always)]
    pub fn af_prio3(&self) -> AF_PRIO3_R {
        AF_PRIO3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AF_PRIO4
    #[inline(always)]
    pub fn af_prio4(&self) -> AF_PRIO4_R {
        AF_PRIO4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AF_PRIO5
    #[inline(always)]
    pub fn af_prio5(&self) -> AF_PRIO5_R {
        AF_PRIO5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AF_PRIO6
    #[inline(always)]
    pub fn af_prio6(&self) -> AF_PRIO6_R {
        AF_PRIO6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AF_PRIO7
    #[inline(always)]
    pub fn af_prio7(&self) -> AF_PRIO7_R {
        AF_PRIO7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR7")
            .field("af_prio0", &self.af_prio0())
            .field("af_prio1", &self.af_prio1())
            .field("af_prio2", &self.af_prio2())
            .field("af_prio3", &self.af_prio3())
            .field("af_prio4", &self.af_prio4())
            .field("af_prio5", &self.af_prio5())
            .field("af_prio6", &self.af_prio6())
            .field("af_prio7", &self.af_prio7())
            .finish()
    }
}
/**GPIO hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOJ:HWCFGR7)*/
pub struct HWCFGR7rs;
impl crate::RegisterSpec for HWCFGR7rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr7::R`](R) reader structure
impl crate::Readable for HWCFGR7rs {}
///`reset()` method sets HWCFGR7 to value 0xffff_ffff
impl crate::Resettable for HWCFGR7rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
