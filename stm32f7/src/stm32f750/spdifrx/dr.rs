#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Field `DR` reader - Parity Error bit"]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `PE` reader - Parity Error bit"]
pub type PE_R = crate::BitReader;
#[doc = "Field `V` reader - Validity bit"]
pub type V_R = crate::BitReader;
#[doc = "Field `U` reader - User bit"]
pub type U_R = crate::BitReader;
#[doc = "Field `C` reader - Channel Status bit"]
pub type C_R = crate::BitReader;
#[doc = "Field `PT` reader - Preamble Type"]
pub type PT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Parity Error bit"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
