///Register `HUFFSYMB50` reader
pub type R = crate::R<HUFFSYMB50rs>;
///Register `HUFFSYMB50` writer
pub type W = crate::W<HUFFSYMB50rs>;
///Field `DATA200` reader - Data 200
pub type DATA200_R = crate::FieldReader;
///Field `DATA200` writer - Data 200
pub type DATA200_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA201` reader - Data 201
pub type DATA201_R = crate::FieldReader;
///Field `DATA201` writer - Data 201
pub type DATA201_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA202` reader - Data 202
pub type DATA202_R = crate::FieldReader;
///Field `DATA202` writer - Data 202
pub type DATA202_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA203` reader - Data 203
pub type DATA203_R = crate::FieldReader;
///Field `DATA203` writer - Data 203
pub type DATA203_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 200
    #[inline(always)]
    pub fn data200(&self) -> DATA200_R {
        DATA200_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 201
    #[inline(always)]
    pub fn data201(&self) -> DATA201_R {
        DATA201_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 202
    #[inline(always)]
    pub fn data202(&self) -> DATA202_R {
        DATA202_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 203
    #[inline(always)]
    pub fn data203(&self) -> DATA203_R {
        DATA203_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB50")
            .field("data200", &self.data200())
            .field("data201", &self.data201())
            .field("data202", &self.data202())
            .field("data203", &self.data203())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 200
    #[inline(always)]
    pub fn data200(&mut self) -> DATA200_W<'_, HUFFSYMB50rs> {
        DATA200_W::new(self, 0)
    }
    ///Bits 8:15 - Data 201
    #[inline(always)]
    pub fn data201(&mut self) -> DATA201_W<'_, HUFFSYMB50rs> {
        DATA201_W::new(self, 8)
    }
    ///Bits 16:23 - Data 202
    #[inline(always)]
    pub fn data202(&mut self) -> DATA202_W<'_, HUFFSYMB50rs> {
        DATA202_W::new(self, 16)
    }
    ///Bits 24:31 - Data 203
    #[inline(always)]
    pub fn data203(&mut self) -> DATA203_W<'_, HUFFSYMB50rs> {
        DATA203_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB50)*/
pub struct HUFFSYMB50rs;
impl crate::RegisterSpec for HUFFSYMB50rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb50::R`](R) reader structure
impl crate::Readable for HUFFSYMB50rs {}
///`write(|w| ..)` method takes [`huffsymb50::W`](W) writer structure
impl crate::Writable for HUFFSYMB50rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB50 to value 0
impl crate::Resettable for HUFFSYMB50rs {}
