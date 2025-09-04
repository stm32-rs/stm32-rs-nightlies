///Register `TDLR` reader
pub type R = crate::R<TDLRrs>;
///Register `TDLR` writer
pub type W = crate::W<TDLRrs>;
///Field `DATA(0-3)` reader - DATA%s
pub type DATA_R = crate::FieldReader;
///Field `DATA(0-3)` writer - DATA%s
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///DATA(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DATA0` field.</div>
    #[inline(always)]
    pub fn data(&self, n: u8) -> DATA_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    ///Iterator for array of:
    ///DATA(0-3)
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = DATA_R> + '_ {
        (0..4).map(move |n| DATA_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    ///Bits 0:7 - DATA0
    #[inline(always)]
    pub fn data0(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA1
    #[inline(always)]
    pub fn data1(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA2
    #[inline(always)]
    pub fn data2(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA3
    #[inline(always)]
    pub fn data3(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDLR")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
impl W {
    ///DATA(0-3)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `DATA0` field.</div>
    #[inline(always)]
    pub fn data(&mut self, n: u8) -> DATA_W<TDLRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        DATA_W::new(self, n * 8)
    }
    ///Bits 0:7 - DATA0
    #[inline(always)]
    pub fn data0(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 0)
    }
    ///Bits 8:15 - DATA1
    #[inline(always)]
    pub fn data1(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 8)
    }
    ///Bits 16:23 - DATA2
    #[inline(always)]
    pub fn data2(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 16)
    }
    ///Bits 24:31 - DATA3
    #[inline(always)]
    pub fn data3(&mut self) -> DATA_W<TDLRrs> {
        DATA_W::new(self, 24)
    }
}
/**CAN_TDL0R

You can [`read`](crate::Reg::read) this register and get [`tdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDLRrs;
impl crate::RegisterSpec for TDLRrs {
    type Ux = u32;
}
///`read()` method returns [`tdlr::R`](R) reader structure
impl crate::Readable for TDLRrs {}
///`write(|w| ..)` method takes [`tdlr::W`](W) writer structure
impl crate::Writable for TDLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDLR to value 0
impl crate::Resettable for TDLRrs {}
