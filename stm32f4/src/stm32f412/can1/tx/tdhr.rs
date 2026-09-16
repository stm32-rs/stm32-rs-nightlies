///Register `TDHR` reader
pub type R = crate::R<TDHRrs>;
///Register `TDHR` writer
pub type W = crate::W<TDHRrs>;
///Field `DATA(4-7)` reader - DATA%s
pub type DATA_R = crate::FieldReader;
///Field `DATA(4-7)` writer - DATA%s
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///DATA(4-7)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DATA4` field.</div>
    #[inline(always)]
    pub fn data(&self, n: u8) -> DATA_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    ///Iterator for array of:
    ///DATA(4-7)
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = DATA_R> + '_ {
        (0..4).map(move |n| DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    ///Bits 0:7 - DATA4
    #[inline(always)]
    pub fn data4(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA5
    #[inline(always)]
    pub fn data5(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA6
    #[inline(always)]
    pub fn data6(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA7
    #[inline(always)]
    pub fn data7(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDHR")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
impl W {
    ///DATA(4-7)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DATA4` field.</div>
    #[inline(always)]
    pub fn data(&mut self, n: u8) -> DATA_W<'_, TDHRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_W::new(self, n * 8)
    }
    ///Bits 0:7 - DATA4
    #[inline(always)]
    pub fn data4(&mut self) -> DATA_W<'_, TDHRrs> {
        DATA_W::new(self, 0)
    }
    ///Bits 8:15 - DATA5
    #[inline(always)]
    pub fn data5(&mut self) -> DATA_W<'_, TDHRrs> {
        DATA_W::new(self, 8)
    }
    ///Bits 16:23 - DATA6
    #[inline(always)]
    pub fn data6(&mut self) -> DATA_W<'_, TDHRrs> {
        DATA_W::new(self, 16)
    }
    ///Bits 24:31 - DATA7
    #[inline(always)]
    pub fn data7(&mut self) -> DATA_W<'_, TDHRrs> {
        DATA_W::new(self, 24)
    }
}
/**mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`tdhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDHRrs;
impl crate::RegisterSpec for TDHRrs {
    type Ux = u32;
}
///`read()` method returns [`tdhr::R`](R) reader structure
impl crate::Readable for TDHRrs {}
///`write(|w| ..)` method takes [`tdhr::W`](W) writer structure
impl crate::Writable for TDHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDHR to value 0
impl crate::Resettable for TDHRrs {}
