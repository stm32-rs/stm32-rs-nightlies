///Register `SCAR_PRG` reader
pub type R = crate::R<SCAR_PRGrs>;
///Register `SCAR_PRG` writer
pub type W = crate::W<SCAR_PRGrs>;
///Field `SEC_AREA_START` reader - Bank 1 secure-only area start configuration bits These bits contain the first block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
pub type SEC_AREA_START_R = crate::FieldReader<u16>;
///Field `SEC_AREA_START` writer - Bank 1 secure-only area start configuration bits These bits contain the first block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
pub type SEC_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SEC_AREA_END` reader - Bank 1 secure-only area end configuration bits These bits contain the last block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
pub type SEC_AREA_END_R = crate::FieldReader<u16>;
///Field `SEC_AREA_END` writer - Bank 1 secure-only area end configuration bits These bits contain the last block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
pub type SEC_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMES` reader - Bank 1 secure access protected erase enable option configuration bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
pub type DMES_R = crate::BitReader;
///Field `DMES` writer - Bank 1 secure access protected erase enable option configuration bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
pub type DMES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bank 1 secure-only area start configuration bits These bits contain the first block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 secure-only area end configuration bits These bits contain the last block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Bank 1 secure access protected erase enable option configuration bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAR_PRG")
            .field("sec_area_start", &self.sec_area_start())
            .field("sec_area_end", &self.sec_area_end())
            .field("dmes", &self.dmes())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bank 1 secure-only area start configuration bits These bits contain the first block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W<'_, SCAR_PRGrs> {
        SEC_AREA_START_W::new(self, 0)
    }
    ///Bits 16:27 - Bank 1 secure-only area end configuration bits These bits contain the last block of 256 bytes of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W<'_, SCAR_PRGrs> {
        SEC_AREA_END_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 secure access protected erase enable option configuration bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
    #[inline(always)]
    pub fn dmes(&mut self) -> DMES_W<'_, SCAR_PRGrs> {
        DMES_W::new(self, 31)
    }
}
/**FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCAR_PRGrs;
impl crate::RegisterSpec for SCAR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`scar_prg::R`](R) reader structure
impl crate::Readable for SCAR_PRGrs {}
///`write(|w| ..)` method takes [`scar_prg::W`](W) writer structure
impl crate::Writable for SCAR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCAR_PRG to value 0
impl crate::Resettable for SCAR_PRGrs {}
