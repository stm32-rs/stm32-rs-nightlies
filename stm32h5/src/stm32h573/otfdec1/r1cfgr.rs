#[doc = "Register `R1CFGR` reader"]
pub type R = crate::R<R1CFGRrs>;
#[doc = "Register `R1CFGR` writer"]
pub type W = crate::W<R1CFGRrs>;
#[doc = "Field `REG_EN` reader - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
pub type REG_EN_R = crate::BitReader;
#[doc = "Field `REG_EN` writer - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
pub type REG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIGLOCK` reader - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
pub type CONFIGLOCK_R = crate::BitReader;
#[doc = "Field `CONFIGLOCK` writer - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
pub type CONFIGLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYLOCK` reader - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
pub type KEYLOCK_R = crate::BitReader;
#[doc = "Field `KEYLOCK` writer - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
pub type KEYLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCRC` reader - region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written."]
pub type KEYCRC_R = crate::FieldReader;
#[doc = "Field `REGx_VERSION` reader - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
pub type REGX_VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `REGx_VERSION` writer - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
pub type REGX_VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - region key 8-bit CRC When KEYLOCK = 0, KEYCRC bitfield is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new computation starts as soon as a new valid sequence is initiated, and KEYCRC is read as zero until a valid sequence is completed. When KEYLOCK = 1, KEYCRC remains unchanged until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Source code is available in . This field is read only. Note: CRC information is updated only after the last bit of the key has been written."]
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
    #[inline(always)]
    pub fn regx_version(&self) -> REGX_VERSION_R {
        REGX_VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - region on-the-fly decryption enable Note: Garbage is decrypted if region context (version, key, nonce) is not valid when this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<R1CFGRrs> {
        REG_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - region config lock Note: This bit is set once. If this bit is set, it can only be reset to 0 if OTFDEC is reset. Setting this bit forces KEYLOCK bit to 1."]
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<R1CFGRrs> {
        CONFIGLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - region key lock Note: This bit is set once: if this bit is set, it can only be reset to 0 if the OTFDEC is reset."]
    #[inline(always)]
    #[must_use]
    pub fn keylock(&mut self) -> KEYLOCK_W<R1CFGRrs> {
        KEYLOCK_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - operating mode This bitfield selects the OTFDEC operating mode for this region: Others: Reserved When MODE ≠ 11, the standard AES encryption mode is activated. When either of the MODE bits are changed, the region key and associated CRC are zeroed."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<R1CFGRrs> {
        MODE_W::new(self, 4)
    }
    #[doc = "Bits 16:31 - region firmware version This 16-bit bitfield must be correctly initialized before the region corresponding REG_EN bit is set in OTFDEC_RxCFGR."]
    #[inline(always)]
    #[must_use]
    pub fn regx_version(&mut self) -> REGX_VERSION_W<R1CFGRrs> {
        REGX_VERSION_W::new(self, 16)
    }
}
#[doc = "OTFDEC region 1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R1CFGRrs;
impl crate::RegisterSpec for R1CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r1cfgr::R`](R) reader structure"]
impl crate::Readable for R1CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`r1cfgr::W`](W) writer structure"]
impl crate::Writable for R1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R1CFGR to value 0"]
impl crate::Resettable for R1CFGRrs {
    const RESET_VALUE: u32 = 0;
}
