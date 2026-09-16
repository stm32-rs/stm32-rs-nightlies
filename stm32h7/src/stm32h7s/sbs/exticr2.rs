///Register `EXTICR2` reader
pub type R = crate::R<EXTICR2rs>;
///Register `EXTICR2` writer
pub type W = crate::W<EXTICR2rs>;
///Field `PC_EXTI4` reader - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI4_R = crate::FieldReader;
///Field `PC_EXTI5` reader - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI5_R = crate::FieldReader;
///Field `PC_EXTI6` reader - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI6_R = crate::FieldReader;
///Field `PC_EXTI7` reader - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
pub type PC_EXTI7_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti4(&self) -> PC_EXTI4_R {
        PC_EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti5(&self) -> PC_EXTI5_R {
        PC_EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti6(&self) -> PC_EXTI6_R {
        PC_EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Port configuration EXTI {1 * 4 + i} This bitfield selects the source input to the EXTI input {1 * 4 + i} used for external interrupt/ event detection. Others: reserved
    #[inline(always)]
    pub fn pc_exti7(&self) -> PC_EXTI7_R {
        PC_EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR2")
            .field("pc_exti4", &self.pc_exti4())
            .field("pc_exti5", &self.pc_exti5())
            .field("pc_exti6", &self.pc_exti6())
            .field("pc_exti7", &self.pc_exti7())
            .finish()
    }
}
impl W {}
/**SBS external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR2)*/
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exticr2::R`](R) reader structure
impl crate::Readable for EXTICR2rs {}
///`write(|w| ..)` method takes [`exticr2::W`](W) writer structure
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2rs {}
