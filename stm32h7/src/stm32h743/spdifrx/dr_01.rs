#[doc = "Register `DR_01` reader"]
pub type R = crate::R<DR_01rs>;
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
#[doc = "Field `DR` reader - Data value"]
pub type DR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Parity Error bit"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Validity bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User bit"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Status bit"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Preamble Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Data value"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_01::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_01rs;
impl crate::RegisterSpec for DR_01rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr_01::R`](R) reader structure"]
impl crate::Readable for DR_01rs {}
#[doc = "`reset()` method sets DR_01 to value 0"]
impl crate::Resettable for DR_01rs {
    const RESET_VALUE: u32 = 0;
}
