#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DCFGrs>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DCFGrs>;
#[doc = "Field `DSPD` reader - Device speed"]
pub type DSPD_R = crate::FieldReader;
#[doc = "Field `DSPD` writer - Device speed"]
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOHSK` reader - Nonzero-length status OUT handshake"]
pub type NZLSOHSK_R = crate::BitReader;
#[doc = "Field `NZLSOHSK` writer - Nonzero-length status OUT handshake"]
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAD` reader - Device address"]
pub type DAD_R = crate::FieldReader;
#[doc = "Field `DAD` writer - Device address"]
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PFIVL` reader - Periodic (micro)frame interval"]
pub type PFIVL_R = crate::FieldReader;
#[doc = "Field `PFIVL` writer - Periodic (micro)frame interval"]
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSCHIVL` reader - Periodic scheduling interval"]
pub type PERSCHIVL_R = crate::FieldReader;
#[doc = "Field `PERSCHIVL` writer - Periodic scheduling interval"]
pub type PERSCHIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    #[must_use]
    pub fn dspd(&mut self) -> DSPD_W<DCFGrs> {
        DSPD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<DCFGrs> {
        DAD_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn pfivl(&mut self) -> PFIVL_W<DCFGrs> {
        PFIVL_W::new(self, 11)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    #[must_use]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<DCFGrs> {
        PERSCHIVL_W::new(self, 24)
    }
}
#[doc = "OTG_HS device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCFGrs;
impl crate::RegisterSpec for DCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DCFGrs {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0220_0000"]
impl crate::Resettable for DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
