#[doc = "Register `FDCAN_GFC` reader"]
pub type R = crate::R<FDCAN_GFCrs>;
#[doc = "Register `FDCAN_GFC` writer"]
pub type W = crate::W<FDCAN_GFCrs>;
#[doc = "Field `RRFE` reader - RRFE"]
pub type RRFE_R = crate::BitReader;
#[doc = "Field `RRFE` writer - RRFE"]
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - RRFS"]
pub type RRFS_R = crate::BitReader;
#[doc = "Field `RRFS` writer - RRFS"]
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` reader - ANFE"]
pub type ANFE_R = crate::FieldReader;
#[doc = "Field `ANFE` writer - ANFE"]
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` reader - ANFS"]
pub type ANFS_R = crate::FieldReader;
#[doc = "Field `ANFS` writer - ANFS"]
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<FDCAN_GFCrs> {
        RRFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<FDCAN_GFCrs> {
        RRFS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<FDCAN_GFCrs> {
        ANFE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<FDCAN_GFCrs> {
        ANFS_W::new(self, 4)
    }
}
#[doc = "Global settings for message ID filtering. The global filter configuration register controls the filter path for standard and extended messages as described in Figure708: Standard message ID filter path and Figure709: Extended message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_gfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_gfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_GFCrs;
impl crate::RegisterSpec for FDCAN_GFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_gfc::R`](R) reader structure"]
impl crate::Readable for FDCAN_GFCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_gfc::W`](W) writer structure"]
impl crate::Writable for FDCAN_GFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_GFC to value 0"]
impl crate::Resettable for FDCAN_GFCrs {
    const RESET_VALUE: u32 = 0;
}
