#[doc = "Register `TDHR` reader"]
pub type R = crate::R<TDHRrs>;
#[doc = "Register `TDHR` writer"]
pub type W = crate::W<TDHRrs>;
#[doc = "Field `DATA(4-7)` reader - DATA%s"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA(4-7)` writer - DATA%s"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "DATA(4-7)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DATA4` field"]
    #[inline(always)]
    pub fn data(&self, n: u8) -> DATA_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DATA(4-7)"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = DATA_R> + '_ {
        (0..4).map(move |n| DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "DATA(4-7)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `DATA4` field"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self, n: u8) -> DATA_W<TDHRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA_W<TDHRrs> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA_W<TDHRrs> {
        DATA_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA_W<TDHRrs> {
        DATA_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA_W<TDHRrs> {
        DATA_W::new(self, 24)
    }
}
#[doc = "mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDHRrs;
impl crate::RegisterSpec for TDHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdhr::R`](R) reader structure"]
impl crate::Readable for TDHRrs {}
#[doc = "`write(|w| ..)` method takes [`tdhr::W`](W) writer structure"]
impl crate::Writable for TDHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDHR to value 0"]
impl crate::Resettable for TDHRrs {
    const RESET_VALUE: u32 = 0;
}
