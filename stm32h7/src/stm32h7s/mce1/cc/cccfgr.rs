///Register `CCCFGR` reader
pub type R = crate::R<CCCFGRrs>;
///Register `CCCFGR` writer
pub type W = crate::W<CCCFGRrs>;
///Field `CCEN` reader - Cipher context enable
pub type CCEN_R = crate::BitReader;
///Field `CCEN` writer - Cipher context enable
pub type CCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCLOCK` reader - Cipher context lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset. Setting this bit forces KEYLOCK bit to 1.
pub type CCLOCK_R = crate::BitReader;
///Field `CCLOCK` writer - Cipher context lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset. Setting this bit forces KEYLOCK bit to 1.
pub type CCLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYLOCK` reader - Key lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset.
pub type KEYLOCK_R = crate::BitReader;
///Field `KEYLOCK` writer - Key lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset.
pub type KEYLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYCRC` reader - Key CRC When KEYLOCK=0, KEYCRC information is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new KEYCRC computation starts as soon as a new valid sequence is initiated. KEYCRC bitfield reads as zero until a valid sequence is completed (after it return the computed CRC value). When GLOCK=1, KEYCRC bitfield always return the computed CRC value until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Note: CRC information is updated, and the key is usable by MCE, only after the last bit of the key has been written. When GLOCK=0 any write to MCE_CCxKEYR registers clears KEYCRC in MCE_CCxCFGR, and makes the cipher context key un-usable (bypass mode is selected instead). To be able to use the key again application must perform this sequence: write to KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). As KEYLOCK=1 all those writes are ignored, so the correct key is used instead.
pub type KEYCRC_R = crate::FieldReader;
///Field `VERSION` reader - Version This 16-bit bitfield must be correctly initialized before CCEN bit is set. Bitfield usage is defined in Section 35.4.6: MCE stream cipher encryption mode.
pub type VERSION_R = crate::FieldReader<u16>;
///Field `VERSION` writer - Version This 16-bit bitfield must be correctly initialized before CCEN bit is set. Bitfield usage is defined in Section 35.4.6: MCE stream cipher encryption mode.
pub type VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Cipher context enable
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Cipher context lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset. Setting this bit forces KEYLOCK bit to 1.
    #[inline(always)]
    pub fn cclock(&self) -> CCLOCK_R {
        CCLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset.
    #[inline(always)]
    pub fn keylock(&self) -> KEYLOCK_R {
        KEYLOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 8:15 - Key CRC When KEYLOCK=0, KEYCRC information is automatically computed by hardware while loading the key of this region in this exact sequence: KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). A new KEYCRC computation starts as soon as a new valid sequence is initiated. KEYCRC bitfield reads as zero until a valid sequence is completed (after it return the computed CRC value). When GLOCK=1, KEYCRC bitfield always return the computed CRC value until the next reset. CRC computation is an 8-bit checksum using the standard CRC-8-CCITT algorithm X8 + X2 + X + 1 (according the convention). Note: CRC information is updated, and the key is usable by MCE, only after the last bit of the key has been written. When GLOCK=0 any write to MCE_CCxKEYR registers clears KEYCRC in MCE_CCxCFGR, and makes the cipher context key un-usable (bypass mode is selected instead). To be able to use the key again application must perform this sequence: write to KEYR0 then KEYR1 then KEYR2 then finally KEYR3 (all written once). As KEYLOCK=1 all those writes are ignored, so the correct key is used instead.
    #[inline(always)]
    pub fn keycrc(&self) -> KEYCRC_R {
        KEYCRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - Version This 16-bit bitfield must be correctly initialized before CCEN bit is set. Bitfield usage is defined in Section 35.4.6: MCE stream cipher encryption mode.
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCFGR")
            .field("ccen", &self.ccen())
            .field("cclock", &self.cclock())
            .field("keylock", &self.keylock())
            .field("keycrc", &self.keycrc())
            .field("version", &self.version())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cipher context enable
    #[inline(always)]
    pub fn ccen(&mut self) -> CCEN_W<'_, CCCFGRrs> {
        CCEN_W::new(self, 0)
    }
    ///Bit 1 - Cipher context lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset. Setting this bit forces KEYLOCK bit to 1.
    #[inline(always)]
    pub fn cclock(&mut self) -> CCLOCK_W<'_, CCCFGRrs> {
        CCLOCK_W::new(self, 1)
    }
    ///Bit 2 - Key lock Note: This bit is set once. If this bit is set, it can only be cleared to 0 if MCE is reset.
    #[inline(always)]
    pub fn keylock(&mut self) -> KEYLOCK_W<'_, CCCFGRrs> {
        KEYLOCK_W::new(self, 2)
    }
    ///Bits 16:31 - Version This 16-bit bitfield must be correctly initialized before CCEN bit is set. Bitfield usage is defined in Section 35.4.6: MCE stream cipher encryption mode.
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W<'_, CCCFGRrs> {
        VERSION_W::new(self, 16)
    }
}
/**MCE cipher context 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`cccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CCCFGRrs;
impl crate::RegisterSpec for CCCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cccfgr::R`](R) reader structure
impl crate::Readable for CCCFGRrs {}
///`write(|w| ..)` method takes [`cccfgr::W`](W) writer structure
impl crate::Writable for CCCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCFGR to value 0
impl crate::Resettable for CCCFGRrs {}
