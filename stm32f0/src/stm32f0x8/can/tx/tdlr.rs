#[doc = "Register `TDLR` reader"]
pub type R = crate::R<TDLRrs>;
#[doc = "Register `TDLR` writer"]
pub type W = crate::W<TDLRrs>;
#[doc = "Field `DATA(0-3)` reader - DATA%s"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA(0-3)` writer - DATA%s"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl W {
    #[doc = "DATA(0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DATA0` field"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self, n: u8) -> DATA_W<TDLRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 24)
    }
}
#[doc = "CAN_TDL0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDLRrs;
impl crate::RegisterSpec for TDLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdlr::R`](R) reader structure"]
impl crate::Readable for TDLRrs {}
#[doc = "`write(|w| ..)` method takes [`tdlr::W`](W) writer structure"]
impl crate::Writable for TDLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDLR to value 0"]
impl crate::Resettable for TDLRrs {
    const RESET_VALUE: u32 = 0;
}
