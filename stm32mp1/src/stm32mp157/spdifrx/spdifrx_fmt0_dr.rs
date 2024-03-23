#[doc = "Register `SPDIFRX_FMT0_DR` reader"]
pub type R = crate::R<SPDIFRX_FMT0_DRrs>;
#[doc = "Field `DR` reader - DR"]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `PE` reader - PE"]
pub type PE_R = crate::BitReader;
#[doc = "Field `V` reader - V"]
pub type V_R = crate::BitReader;
#[doc = "Field `U` reader - U"]
pub type U_R = crate::BitReader;
#[doc = "Field `C` reader - C"]
pub type C_R = crate::BitReader;
#[doc = "Field `PT` reader - PT"]
pub type PT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - V"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - U"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - C"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_fmt0_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_FMT0_DRrs;
impl crate::RegisterSpec for SPDIFRX_FMT0_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_fmt0_dr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_FMT0_DRrs {}
#[doc = "`reset()` method sets SPDIFRX_FMT0_DR to value 0"]
impl crate::Resettable for SPDIFRX_FMT0_DRrs {
    const RESET_VALUE: u32 = 0;
}
