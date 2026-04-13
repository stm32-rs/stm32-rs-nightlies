///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `IN1` reader - IN1
pub type IN1_R = crate::BitReader;
///Field `IN1` writer - IN1
pub type IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN2` reader - IN2
pub type IN2_R = crate::BitReader;
///Field `IN2` writer - IN2
pub type IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN1_2_1` reader - IN1_2_1
pub type IN1_2_1_R = crate::FieldReader;
///Field `IN1_2_1` writer - IN1_2_1
pub type IN1_2_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IN2_2_1` reader - IN2_2_1
pub type IN2_2_1_R = crate::FieldReader;
///Field `IN2_2_1` writer - IN2_2_1
pub type IN2_2_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - IN1
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IN2
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - IN1_2_1
    #[inline(always)]
    pub fn in1_2_1(&self) -> IN1_2_1_R {
        IN1_2_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - IN2_2_1
    #[inline(always)]
    pub fn in2_2_1(&self) -> IN2_2_1_R {
        IN2_2_1_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("in1", &self.in1())
            .field("in2", &self.in2())
            .field("in1_2_1", &self.in1_2_1())
            .field("in2_2_1", &self.in2_2_1())
            .finish()
    }
}
impl W {
    ///Bit 0 - IN1
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W<'_, ORrs> {
        IN1_W::new(self, 0)
    }
    ///Bit 1 - IN2
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W<'_, ORrs> {
        IN2_W::new(self, 1)
    }
    ///Bits 2:3 - IN1_2_1
    #[inline(always)]
    pub fn in1_2_1(&mut self) -> IN1_2_1_W<'_, ORrs> {
        IN1_2_1_W::new(self, 2)
    }
    ///Bits 4:5 - IN2_2_1
    #[inline(always)]
    pub fn in2_2_1(&mut self) -> IN2_2_1_W<'_, ORrs> {
        IN2_2_1_W::new(self, 4)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#LPTIM1:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
