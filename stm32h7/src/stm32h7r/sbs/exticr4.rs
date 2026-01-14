///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
///Field `PC_EXTI12` reader - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI12_R = crate::FieldReader;
///Field `PC_EXTI13` reader - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI13_R = crate::FieldReader;
///Field `PC_EXTI14` reader - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI14_R = crate::FieldReader;
///Field `PC_EXTI15` reader - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI15_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti12(&self) -> PC_EXTI12_R {
        PC_EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti13(&self) -> PC_EXTI13_R {
        PC_EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti14(&self) -> PC_EXTI14_R {
        PC_EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port configuration EXTI {3 * 4 + i} This bitfield selects the source input to the EXTI input {3 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti15(&self) -> PC_EXTI15_R {
        PC_EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("pc_exti12", &self.pc_exti12())
            .field("pc_exti13", &self.pc_exti13())
            .field("pc_exti14", &self.pc_exti14())
            .field("pc_exti15", &self.pc_exti15())
            .finish()
    }
}
impl W {}
/**SBS external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:EXTICR4)*/
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exticr4::R`](R) reader structure
impl crate::Readable for EXTICR4rs {}
///`write(|w| ..)` method takes [`exticr4::W`](W) writer structure
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4rs {}
