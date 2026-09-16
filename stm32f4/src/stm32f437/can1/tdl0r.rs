///Register `TDL0R` reader
pub type R = crate::R<TDL0Rrs>;
///Register `TDL0R` writer
pub type W = crate::W<TDL0Rrs>;
///Field `DATA0` reader - DATA0
pub type DATA0_R = crate::FieldReader;
///Field `DATA0` writer - DATA0
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA1` reader - DATA1
pub type DATA1_R = crate::FieldReader;
///Field `DATA1` writer - DATA1
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA2` reader - DATA2
pub type DATA2_R = crate::FieldReader;
///Field `DATA2` writer - DATA2
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA3` reader - DATA3
pub type DATA3_R = crate::FieldReader;
///Field `DATA3` writer - DATA3
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DATA0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DATA1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DATA2
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DATA3
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDL0R")
            .field("data3", &self.data3())
            .field("data2", &self.data2())
            .field("data1", &self.data1())
            .field("data0", &self.data0())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DATA0
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W<'_, TDL0Rrs> {
        DATA0_W::new(self, 0)
    }
    ///Bits 8:15 - DATA1
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W<'_, TDL0Rrs> {
        DATA1_W::new(self, 8)
    }
    ///Bits 16:23 - DATA2
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W<'_, TDL0Rrs> {
        DATA2_W::new(self, 16)
    }
    ///Bits 24:31 - DATA3
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W<'_, TDL0Rrs> {
        DATA3_W::new(self, 24)
    }
}
/**mailbox data low register

You can [`read`](crate::Reg::read) this register and get [`tdl0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdl0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#CAN1:TDL0R)*/
pub struct TDL0Rrs;
impl crate::RegisterSpec for TDL0Rrs {
    type Ux = u32;
}
///`read()` method returns [`tdl0r::R`](R) reader structure
impl crate::Readable for TDL0Rrs {}
///`write(|w| ..)` method takes [`tdl0r::W`](W) writer structure
impl crate::Writable for TDL0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDL0R to value 0
impl crate::Resettable for TDL0Rrs {}
