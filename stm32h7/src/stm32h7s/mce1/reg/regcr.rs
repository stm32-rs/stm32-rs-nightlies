///Register `REGCR` reader
pub type R = crate::R<REGCRrs>;
///Register `REGCR` writer
pub type W = crate::W<REGCRrs>;
///Field `BREN` reader - Base region enable BREN cannot be set if BADDRSTART > BADDREND.
pub type BREN_R = crate::BitReader;
///Field `BREN` writer - Base region enable BREN cannot be set if BADDRSTART > BADDREND.
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTXID` reader - Context ID This bitfield defines the cryptographic context used by the cipher engine assigned to this region. If ENC=00 bitfield CTXID is ignored. If BREN is set write to this bitfield is ignored. If ENC=01 the key stored in MCE_CC1KEYR is used by the stream cipher. The nonce in MCE_CC1NRx registers and the version in MCE_CC1CR register are also used. If ENC=01 the key stored in MCE_CC2KEYR is used by the stream cipher. The nonce in MCE_CC2NRx registers and the version in MCE_CC2CR register are also used.
pub type CTXID_R = crate::FieldReader;
///Field `CTXID` writer - Context ID This bitfield defines the cryptographic context used by the cipher engine assigned to this region. If ENC=00 bitfield CTXID is ignored. If BREN is set write to this bitfield is ignored. If ENC=01 the key stored in MCE_CC1KEYR is used by the stream cipher. The nonce in MCE_CC1NRx registers and the version in MCE_CC1CR register are also used. If ENC=01 the key stored in MCE_CC2KEYR is used by the stream cipher. The nonce in MCE_CC2NRx registers and the version in MCE_CC2CR register are also used.
pub type CTXID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ENC` reader - Encrypted region Those bits are taken into account only if BREN is set and if the corresponding encryption feature is available in the MCE instance (see Section 35.3: MCE implementation). Write to those bits is ignored if BREN is set.
pub type ENC_R = crate::FieldReader;
///Field `ENC` writer - Encrypted region Those bits are taken into account only if BREN is set and if the corresponding encryption feature is available in the MCE instance (see Section 35.3: MCE implementation). Write to those bits is ignored if BREN is set.
pub type ENC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRIV` reader - Privileged region This bit is taken into account only if BREN is set.
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Privileged region This bit is taken into account only if BREN is set.
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Base region enable BREN cannot be set if BADDRSTART > BADDREND.
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    ///Bits 9:10 - Context ID This bitfield defines the cryptographic context used by the cipher engine assigned to this region. If ENC=00 bitfield CTXID is ignored. If BREN is set write to this bitfield is ignored. If ENC=01 the key stored in MCE_CC1KEYR is used by the stream cipher. The nonce in MCE_CC1NRx registers and the version in MCE_CC1CR register are also used. If ENC=01 the key stored in MCE_CC2KEYR is used by the stream cipher. The nonce in MCE_CC2NRx registers and the version in MCE_CC2CR register are also used.
    #[inline(always)]
    pub fn ctxid(&self) -> CTXID_R {
        CTXID_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 14:15 - Encrypted region Those bits are taken into account only if BREN is set and if the corresponding encryption feature is available in the MCE instance (see Section 35.3: MCE implementation). Write to those bits is ignored if BREN is set.
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Privileged region This bit is taken into account only if BREN is set.
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCR")
            .field("bren", &self.bren())
            .field("ctxid", &self.ctxid())
            .field("enc", &self.enc())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - Base region enable BREN cannot be set if BADDRSTART > BADDREND.
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W<'_, REGCRrs> {
        BREN_W::new(self, 0)
    }
    ///Bits 9:10 - Context ID This bitfield defines the cryptographic context used by the cipher engine assigned to this region. If ENC=00 bitfield CTXID is ignored. If BREN is set write to this bitfield is ignored. If ENC=01 the key stored in MCE_CC1KEYR is used by the stream cipher. The nonce in MCE_CC1NRx registers and the version in MCE_CC1CR register are also used. If ENC=01 the key stored in MCE_CC2KEYR is used by the stream cipher. The nonce in MCE_CC2NRx registers and the version in MCE_CC2CR register are also used.
    #[inline(always)]
    pub fn ctxid(&mut self) -> CTXID_W<'_, REGCRrs> {
        CTXID_W::new(self, 9)
    }
    ///Bits 14:15 - Encrypted region Those bits are taken into account only if BREN is set and if the corresponding encryption feature is available in the MCE instance (see Section 35.3: MCE implementation). Write to those bits is ignored if BREN is set.
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W<'_, REGCRrs> {
        ENC_W::new(self, 14)
    }
    ///Bit 16 - Privileged region This bit is taken into account only if BREN is set.
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, REGCRrs> {
        PRIV_W::new(self, 16)
    }
}
/**Region configuration register

You can [`read`](crate::Reg::read) this register and get [`regcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REGCRrs;
impl crate::RegisterSpec for REGCRrs {
    type Ux = u32;
}
///`read()` method returns [`regcr::R`](R) reader structure
impl crate::Readable for REGCRrs {}
///`write(|w| ..)` method takes [`regcr::W`](W) writer structure
impl crate::Writable for REGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGCR to value 0
impl crate::Resettable for REGCRrs {}
