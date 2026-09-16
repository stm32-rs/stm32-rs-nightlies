///Register `SCAR_CUR` reader
pub type R = crate::R<SCAR_CURrs>;
///Field `SEC_AREA_START` reader - Bank 1 secure-only area start status bits These bits contain the first 256 bytes of block of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
pub type SEC_AREA_START_R = crate::FieldReader<u16>;
///Field `SEC_AREA_END` reader - Bank 1 secure-only area end status bits These bits contain the last 256-byte block of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
pub type SEC_AREA_END_R = crate::FieldReader<u16>;
///Field `DMES` reader - Bank 1 secure access protected erase enable option status bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
pub type DMES_R = crate::BitReader;
impl R {
    ///Bits 0:11 - Bank 1 secure-only area start status bits These bits contain the first 256 bytes of block of the secure-only area in bank 1. If this address is equal to SEC_AREA_END1, the whole bank 1 is secure access only. If this address is higher than SEC_AREA_END1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 secure-only area end status bits These bits contain the last 256-byte block of the secure-only area in bank 1. If this address is equal to SEC_AREA_START1, the whole bank 1 is secure access only. If this address is lower than SEC_AREA_START1, no protection is set on bank 1.
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Bank 1 secure access protected erase enable option status bit If DMES1 is set to 1, the secure access only area in bank 1 is erased when a protection level regression (change from level 1 to 0) or a bank erase with protection removal occurs.
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAR_CUR")
            .field("sec_area_start", &self.sec_area_start())
            .field("sec_area_end", &self.sec_area_end())
            .field("dmes", &self.dmes())
            .finish()
    }
}
/**FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCAR_CURrs;
impl crate::RegisterSpec for SCAR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`scar_cur::R`](R) reader structure
impl crate::Readable for SCAR_CURrs {}
///`reset()` method sets SCAR_CUR to value 0
impl crate::Resettable for SCAR_CURrs {}
