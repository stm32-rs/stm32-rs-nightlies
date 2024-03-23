#[doc = "Register `PCR%s` reader"]
pub type R = crate::R<PCRrs>;
#[doc = "Register `PCR%s` writer"]
pub type W = crate::W<PCRrs>;
#[doc = "PWAITEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWAITEN {
    #[doc = "0: Wait feature disabled"]
    Disabled = 0,
    #[doc = "1: Wait feature enabled"]
    Enabled = 1,
}
impl From<PWAITEN> for bool {
    #[inline(always)]
    fn from(variant: PWAITEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader<PWAITEN>;
impl PWAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWAITEN {
        match self.bits {
            false => PWAITEN::Disabled,
            true => PWAITEN::Enabled,
        }
    }
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWAITEN::Disabled
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWAITEN::Enabled
    }
}
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG, PWAITEN>;
impl<'a, REG> PWAITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWAITEN::Disabled)
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWAITEN::Enabled)
    }
}
#[doc = "PBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBKEN {
    #[doc = "0: Corresponding memory bank is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    Enabled = 1,
}
impl From<PBKEN> for bool {
    #[inline(always)]
    fn from(variant: PBKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader<PBKEN>;
impl PBKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PBKEN {
        match self.bits {
            false => PBKEN::Disabled,
            true => PBKEN::Enabled,
        }
    }
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PBKEN::Disabled
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PBKEN::Enabled
    }
}
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG, PBKEN>;
impl<'a, REG> PBKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PBKEN::Disabled)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PBKEN::Enabled)
    }
}
#[doc = "PTYP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTYP {
    #[doc = "1: NAND Flash"]
    Nandflash = 1,
}
impl From<PTYP> for bool {
    #[inline(always)]
    fn from(variant: PTYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTYP` reader - PTYP"]
pub type PTYP_R = crate::BitReader<PTYP>;
impl PTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTYP> {
        match self.bits {
            true => Some(PTYP::Nandflash),
            _ => None,
        }
    }
    #[doc = "NAND Flash"]
    #[inline(always)]
    pub fn is_nandflash(&self) -> bool {
        *self == PTYP::Nandflash
    }
}
#[doc = "Field `PTYP` writer - PTYP"]
pub type PTYP_W<'a, REG> = crate::BitWriter<'a, REG, PTYP>;
impl<'a, REG> PTYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NAND Flash"]
    #[inline(always)]
    pub fn nandflash(self) -> &'a mut crate::W<REG> {
        self.variant(PTYP::Nandflash)
    }
}
#[doc = "PWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWID {
    #[doc = "0: External memory device width 8 bits"]
    Bits8 = 0,
    #[doc = "1: External memory device width 16 bits"]
    Bits16 = 1,
}
impl From<PWID> for u8 {
    #[inline(always)]
    fn from(variant: PWID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWID {
    type Ux = u8;
}
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader<PWID>;
impl PWID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWID> {
        match self.bits {
            0 => Some(PWID::Bits8),
            1 => Some(PWID::Bits16),
            _ => None,
        }
    }
    #[doc = "External memory device width 8 bits"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWID::Bits8
    }
    #[doc = "External memory device width 16 bits"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWID::Bits16
    }
}
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWID>;
impl<'a, REG> PWID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External memory device width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PWID::Bits8)
    }
    #[doc = "External memory device width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PWID::Bits16)
    }
}
#[doc = "ECCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCEN {
    #[doc = "0: ECC logic is disabled and reset"]
    Disabled = 0,
    #[doc = "1: ECC logic is enabled"]
    Enabled = 1,
}
impl From<ECCEN> for bool {
    #[inline(always)]
    fn from(variant: ECCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader<ECCEN>;
impl ECCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCEN {
        match self.bits {
            false => ECCEN::Disabled,
            true => ECCEN::Enabled,
        }
    }
    #[doc = "ECC logic is disabled and reset"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCEN::Disabled
    }
    #[doc = "ECC logic is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCEN::Enabled
    }
}
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG, ECCEN>;
impl<'a, REG> ECCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC logic is disabled and reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCEN::Disabled)
    }
    #[doc = "ECC logic is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ECCEN::Enabled)
    }
}
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "ECCPS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCPS {
    #[doc = "0: ECC page size 256 bytes"]
    Bytes256 = 0,
    #[doc = "1: ECC page size 512 bytes"]
    Bytes512 = 1,
    #[doc = "2: ECC page size 1024 bytes"]
    Bytes1024 = 2,
    #[doc = "3: ECC page size 2048 bytes"]
    Bytes2048 = 3,
    #[doc = "4: ECC page size 4096 bytes"]
    Bytes4096 = 4,
    #[doc = "5: ECC page size 8192 bytes"]
    Bytes8192 = 5,
}
impl From<ECCPS> for u8 {
    #[inline(always)]
    fn from(variant: ECCPS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ECCPS {
    type Ux = u8;
}
#[doc = "Field `ECCPS` reader - ECCPS"]
pub type ECCPS_R = crate::FieldReader<ECCPS>;
impl ECCPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ECCPS> {
        match self.bits {
            0 => Some(ECCPS::Bytes256),
            1 => Some(ECCPS::Bytes512),
            2 => Some(ECCPS::Bytes1024),
            3 => Some(ECCPS::Bytes2048),
            4 => Some(ECCPS::Bytes4096),
            5 => Some(ECCPS::Bytes8192),
            _ => None,
        }
    }
    #[doc = "ECC page size 256 bytes"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == ECCPS::Bytes256
    }
    #[doc = "ECC page size 512 bytes"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == ECCPS::Bytes512
    }
    #[doc = "ECC page size 1024 bytes"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == ECCPS::Bytes1024
    }
    #[doc = "ECC page size 2048 bytes"]
    #[inline(always)]
    pub fn is_bytes2048(&self) -> bool {
        *self == ECCPS::Bytes2048
    }
    #[doc = "ECC page size 4096 bytes"]
    #[inline(always)]
    pub fn is_bytes4096(&self) -> bool {
        *self == ECCPS::Bytes4096
    }
    #[doc = "ECC page size 8192 bytes"]
    #[inline(always)]
    pub fn is_bytes8192(&self) -> bool {
        *self == ECCPS::Bytes8192
    }
}
#[doc = "Field `ECCPS` writer - ECCPS"]
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ECCPS>;
impl<'a, REG> ECCPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECC page size 256 bytes"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes256)
    }
    #[doc = "ECC page size 512 bytes"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes512)
    }
    #[doc = "ECC page size 1024 bytes"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes1024)
    }
    #[doc = "ECC page size 2048 bytes"]
    #[inline(always)]
    pub fn bytes2048(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes2048)
    }
    #[doc = "ECC page size 4096 bytes"]
    #[inline(always)]
    pub fn bytes4096(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes4096)
    }
    #[doc = "ECC page size 8192 bytes"]
    #[inline(always)]
    pub fn bytes8192(self) -> &'a mut crate::W<REG> {
        self.variant(ECCPS::Bytes8192)
    }
}
impl R {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<PCRrs> {
        PTYP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<PCRrs> {
        TAR_W::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
#[doc = "PC Card/NAND Flash control register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCRrs {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR%s to value 0x18"]
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
