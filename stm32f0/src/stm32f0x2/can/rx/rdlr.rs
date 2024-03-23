#[doc = "Register `RDLR` reader"]
pub type R = crate::R<RDLRrs>;
#[doc = "Field `DATA(0-3)` reader - DATA%s"]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "DATA(0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DATA0` field"]
    #[inline(always)]
    pub fn data(&self, n: u8) -> DATA_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DATA(0-3)"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = DATA_R> + '_ {
        (0..4).map(move |n| DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CAN_RDL0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdlr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDLRrs;
impl crate::RegisterSpec for RDLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdlr::R`](R) reader structure"]
impl crate::Readable for RDLRrs {}
#[doc = "`reset()` method sets RDLR to value 0"]
impl crate::Resettable for RDLRrs {
    const RESET_VALUE: u32 = 0;
}
