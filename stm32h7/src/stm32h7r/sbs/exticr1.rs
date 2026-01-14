///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
///Field `PC_EXTI0` reader - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI0_R = crate::FieldReader;
///Field `PC_EXTI1` reader - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI1_R = crate::FieldReader;
///Field `PC_EXTI2` reader - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI2_R = crate::FieldReader;
///Field `PC_EXTI3` reader - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI3_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti0(&self) -> PC_EXTI0_R {
        PC_EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti1(&self) -> PC_EXTI1_R {
        PC_EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti2(&self) -> PC_EXTI2_R {
        PC_EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port configuration EXTI {0 * 4 + i} This bitfield selects the source input to the EXTI input {0 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti3(&self) -> PC_EXTI3_R {
        PC_EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR1")
            .field("pc_exti0", &self.pc_exti0())
            .field("pc_exti1", &self.pc_exti1())
            .field("pc_exti2", &self.pc_exti2())
            .field("pc_exti3", &self.pc_exti3())
            .finish()
    }
}
impl W {}
/**SBS external interrupt configuration register 0

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:EXTICR1)*/
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exticr1::R`](R) reader structure
impl crate::Readable for EXTICR1rs {}
///`write(|w| ..)` method takes [`exticr1::W`](W) writer structure
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1rs {}
